#![allow(warnings, unused)]

use colored::Colorize;
use csv::Reader;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;

// PREP - Problem, Return, Example, Pseudocode

// TODO: Init commit

// read from path
// make an import function
// get a list
// map over to format our info
// write some matching cases
// filter by those cases into:
// final list
// calc the sum

// Quarterly like ClearDefense is an issue.........
// Items that recurr only x may need to be suggested, or if they occur 4 times,
// we have to split them up

// Slider on how many must match ( 2 - 6)
// Move things out into modules

// mark as quartlery, half yearly, yearly, etc.

// naive matching

// measure first & last date, to determine months span
// can't assume that 2 transactions in 6 months will repeat every year

//  Transaction Date,Post Date,Description,Category,Type,Amount,Memo
// Scrub the doc for anything like a regex of "Desc, Descript, etc. all variation"

// Add | bar on the keyboard on the same place as shift
// Add & to help with borrowing

#[derive(Debug, Deserialize)]
struct CSVTransaction {
    Description: String,
    Amount: f64,
}

#[derive(Debug, Clone)]
struct Transaction {
    full_desc: String,
    amount: f64,
    count: i32,
    frequency: i32,
    checked: bool,
}

fn main() -> Result<(), csv::Error> {
    let mut reader: Vec<Reader<File>> = vec![
        Reader::from_path("test/chase_sapphire.csv").unwrap(),
        Reader::from_path("test/chase_amz.csv").unwrap(),
    ];

    fn list_from_csvs(reader: &mut Vec<Reader<File>>) -> Vec<CSVTransaction> {
        let mut list: Vec<CSVTransaction> = Vec::new();
        for single_reader in reader.iter_mut() {
            for transaction in single_reader.deserialize() {
                list.push(transaction.unwrap());
            }
        }
        return list;
    }

    fn format_identifier(s: &str) -> &str {
        if let Some(i) = s.find('#') {
            return &s[..i];
        }
        if let Some(i) = s.find('*') {
            return &s[..i];
        }
        s
    }

    // TODO: This is a mess, please improve
    fn get_matched_from_csv(
        transactions: &Vec<CSVTransaction>,
        transaction_map: &mut HashMap<String, Transaction>,
    ) -> HashMap<String, Transaction> {
        transactions
            .iter()
            .for_each(|transaction: &CSVTransaction| {
                if transaction.Amount > 0.0 {
                    return;
                };

                let identifier = format_identifier(&transaction.Description);
                let full_desc = transaction.Description.clone();
                let hash_key = (identifier.to_string() + &transaction.Amount.abs().to_string())
                    .replace('.', ""); // TODO: Sanitize
                let new_transaction = transaction_map.entry(hash_key).or_insert(Transaction {
                    full_desc,
                    amount: transaction.Amount.abs(),
                    count: 0,
                    frequency: 12,
                    checked: true,
                });
                new_transaction.count += 1;
            });

        return transaction_map.clone();
    }

    // TODO: One filter that takes multiple arguements?
    fn custom_filter(transaction: &Transaction, detect: &i32) -> bool {
        transaction.count >= *detect && transaction.checked
    }

    // Main Work
    let mut transactions_map: HashMap<String, Transaction> = HashMap::new();
    let list = list_from_csvs(&mut reader);

    let all_transactions = get_matched_from_csv(&list, &mut transactions_map);
    const detect_count: i32 = 6;

    let recurring: HashMap<_, _> = all_transactions
        .into_iter()
        .filter(|(_, transaction)| custom_filter(&transaction, &detect_count))
        .collect();

    // Stateless computed values, TODO: should use frequency
    let monthly = recurring.values().fold(0.0, |acc, x| acc + x.amount);

    println!(
        "{}, {}",
        format!("Monthly: ${:.2}", monthly).red().bold(),
        format!("Per year: ${:.2}", monthly * 12.0).yellow().bold()
    );
    println!("{}", format!("Current: {:?}", recurring).green());

    Ok(())
}
