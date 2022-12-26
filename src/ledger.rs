use colored::Colorize;
use csv::Reader;
use error_chain::error_chain;
use glob::{glob_with, MatchOptions};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct CSVTransaction {
    pub Description: String,
    pub Amount: f64,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub full_desc: String,
    pub amount: f64,
    pub count: i32,
    pub frequency: i32,
    pub checked: bool,
}

error_chain! {
    foreign_links {
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

pub fn get_hello() -> String {
    "hello world from ledger".to_string()
}

pub fn filter_transactions_by(transaction: &Transaction, detect: &i32) -> bool {
    transaction.count >= *detect && transaction.checked
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

fn files_in_dir(dir: &str) -> Result<Vec<CSVTransaction>> {
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    let find_in: String = format!("{}/*.csv", dir);
    let mut list: Vec<Reader<File>> = vec![];

    // all this loopin' is goofy, so refactor it.
    for entry in glob_with(&find_in, options)? {
        match entry {
            Ok(path) => list.push(Reader::from_path(path).unwrap()),
            Err(e) => println!("{:?}", e),
        };
    }

    fn list_from_csvs(reader: &mut Vec<Reader<File>>) -> Vec<CSVTransaction> {
        let mut list: Vec<CSVTransaction> = Vec::new();
        for single_reader in reader.iter_mut() {
            for transaction in single_reader.deserialize() {
                list.push(transaction.unwrap());
            }
        }
        return list;
    }

    Ok(list_from_csvs(&mut list))
}

// TODO: This is a mess, please improve
// This function is just the worst. I don't like it. It needs a lot better.
pub fn match_transactions(
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
            let hash_key =
                (identifier.to_string() + &transaction.Amount.abs().to_string()).replace('.', ""); // TODO: Sanitize
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

#[derive(Default)]
pub struct MonthlyTransactions {
    pub transactions: HashMap<String, Transaction>,
    pub monthly: f64,
}

pub fn get_monthly_total(transactions: &HashMap<String, Transaction>) -> f64 {
    // TODO: Filter these to only count checked!
    transactions
        .into_iter()
        .filter(|(_, transaction)| transaction.checked)
        .fold(0.0, |acc, (x, y)| acc + y.amount)
}

pub fn get_monthly_transactions(dir: &String) -> HashMap<String, Transaction> {
    // Main Work
    let mut transactions_map: HashMap<String, Transaction> = HashMap::new();
    // henious
    let all_transactions =
        match_transactions(&mut files_in_dir(dir).unwrap(), &mut transactions_map);

    // if (all_transactions.is_empty()) {
    //     println!(
    //         "{}",
    //         format!("No transactions found in directory: '{}' ", "NONE")
    //             .red()
    //             .bold(),
    //     );
    //     return Ok(());
    // }

    // TODO: This will be an arg or slider
    let detect_count: i32 = 6;

    let recurring: HashMap<String, Transaction> = all_transactions
        .into_iter()
        .filter(|(_, transaction)| filter_transactions_by(&transaction, &detect_count))
        .collect();

    // // Stateless computed values, TODO: should use frequency
    let monthly = recurring.values().fold(0.0, |acc, x| acc + x.amount);

    // println!(
    //     "{}, {}",
    //     format!("Monthly: ${:.2}", monthly).red().bold(),
    //     format!("Per year: ${:.2}", monthly * 12.0).yellow().bold()
    // );
    // println!("{}", format!("Current: {:?}", recurring).green());

    return recurring;
    // return MonthlyTransactions {
    //     transactions: recurring,
    //     monthly: monthly,
    // };
}
