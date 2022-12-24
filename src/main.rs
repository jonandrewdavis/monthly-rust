#![allow(warnings, unused)]

use colored::Colorize;
use csv::Reader;
use eframe::egui::{self, RichText};
use eframe::epaint::FontId;
use error_chain::error_chain;
use glob::{glob_with, MatchOptions};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;

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

error_chain! {
    foreign_links {
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

// Result<(), csv::Error>
fn main() -> Result<()> {
    let dir = "test";
    // TODO: One filter that takes multiple arguements?
    fn custom_filter(transaction: &Transaction, detect: &i32) -> bool {
        transaction.count >= *detect && transaction.checked
    }

    // Main Work
    let mut transactions_map: HashMap<String, Transaction> = HashMap::new();
    let all_transactions =
        get_matched_from_csv(&mut find_files_in_dir(dir)?, &mut transactions_map);

    if (all_transactions.is_empty()) {
        println!(
            "{}",
            format!("No transactions found in directory: '{}' ", dir)
                .red()
                .bold(),
        );
        return Ok(());
    }

    // TODO: This will be an arg or slider
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

    // GUI

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Native file dialogs and drag-and-drop files",
        options,
        Box::new(|_cc| Box::new(MonthlyApp::default())),
    );

    Ok(())
}

/// UH
fn find_files_in_dir(dir: &str) -> Result<Vec<CSVTransaction>> {
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
struct MonthlyApp {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

impl eframe::App for MonthlyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new("monthly").font(FontId::proportional(40.0)));
            ui.label("Drag-and-drop files onto the window!");

            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                let test = "lol";
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                    ui.label(test);
                });
            }

            // Show dropped files (if any):
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");

                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };
                        if let Some(bytes) = &file.bytes {
                            use std::fmt::Write as _;
                            write!(info, " ({} bytes)", bytes.len()).ok();
                        }
                        ui.label(info);
                    }
                });
            }
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        if !ctx.input().raw.dropped_files.is_empty() {
            self.dropped_files = ctx.input().raw.dropped_files.clone();
        }
    }
}

/// Preview hovering files:
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input().raw.hovered_files.is_empty() {
        let mut text = "Dropping files:\n".to_owned();
        for file in &ctx.input().raw.hovered_files {
            if let Some(path) = &file.path {
                write!(text, "\n{}", path.display()).ok();
            } else if !file.mime.is_empty() {
                write!(text, "\n{}", file.mime).ok();
            } else {
                text += "\n???";
            }
        }

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.input().screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
