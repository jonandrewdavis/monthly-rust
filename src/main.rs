#![allow(warnings, unused)]

use colored::Colorize;
use csv::Reader;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;

// PREP - Problem, Return, Example, Pseudocode

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

#[derive(Debug)]
struct Transaction {
    full_desc: String,
    amount: f64,
    count: u32,
}

fn main() -> Result<(), csv::Error> {
    let mut reader: Reader<File> = Reader::from_path("test/chase_sapphire.csv").unwrap();

    let mut list: Vec<CSVTransaction> = Vec::new();
    let mut transactions_map: HashMap<String, Transaction> = HashMap::new();

    // Deserialize the CSV records into a vector of `CSVTransaction`s.
    for transaction in reader.deserialize() {
        let transaction: CSVTransaction = transaction?;
        list.push(transaction);
    }

    // TODO: This is a mess, please improve
    fn get_matched_from_csv(
        transactions: &Vec<CSVTransaction>,
        transaction_map: &mut HashMap<String, Transaction>,
    ) {
        transactions
            .iter()
            .for_each(|transaction: &CSVTransaction| {
                if transaction.Amount > 0.0 {
                    return;
                };
                let desc = transaction.Description.clone().to_lowercase()[0..4].to_string();
                let full_desc = transaction.Description.clone();
                let hash_key = transaction.Amount.to_string() + &desc;
                let new_transaction = transaction_map.entry(hash_key).or_insert(Transaction {
                    full_desc,
                    amount: transaction.Amount.abs(),
                    count: 0,
                });
                new_transaction.count += 1;
            });

        let recurring: HashMap<_, _> = transaction_map
            .into_iter()
            .filter(|(_, transaction)| transaction.count > 6)
            .collect();

        let monthly = recurring.values().fold(0.0, |acc, x| acc + x.amount);

        println!("Monthly: {:.2}, Total: {:.2}", monthly, monthly * 12.0);
        println!("Current: {:?}", recurring.values());
    }

    get_matched_from_csv(&list, &mut transactions_map);

    Ok(())
}
