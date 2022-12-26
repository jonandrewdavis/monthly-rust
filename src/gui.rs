#![allow(warnings, unused)]

use eframe::egui::{self, RichText};
use eframe::epaint::FontId;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use crate::ledger::{
    filter_transactions_by, get_hello, get_monthly_total, get_monthly_transactions,
    match_transactions, CSVTransaction, MonthlyTransactions, Transaction,
};

#[derive(Default)]
pub struct MonthlyApp {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    picked_folder: Option<String>,
    transactions: HashMap<String, Transaction>,
}

impl eframe::App for MonthlyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new("monthly").font(FontId::proportional(40.0)));
            ui.label("Drag-and-drop files onto the window!");

            let WORKING_DIR = env::current_dir().unwrap();

            // if ui.button("Open file ...").clicked() {
            //     if let Some(path) = rfd::FileDialog::new()
            //         .add_filter(".CSV", &["CSV"])
            //         .set_directory(&WORKING_DIR)
            //         .pick_file()
            //     {
            //         self.picked_path = Some(path.display().to_string());
            //     }
            // }

            ui.label("Choose a folder containing CSV files");
            if ui.button("Open Folder ...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .set_directory(&WORKING_DIR)
                    .pick_folder()
                {
                    match path.display().to_string() {
                        path => {
                            self.transactions = get_monthly_transactions(&path);
                            self.picked_folder = Some(path);
                        }
                    }
                }
            }

            if let Some(picked_folder) = &self.picked_folder {
                ui.horizontal(|ui| {
                    ui.label("Picked folder:");
                    ui.monospace(picked_folder);
                });
            };

            if self.transactions.len() > 0 {
                ui.vertical(|ui| {
                    for (id, mut transaction) in self.transactions.clone() {
                        ui.horizontal(|ui| {
                            ui.label(format!("Desc: {}", transaction.full_desc));
                            ui.label(format!("Amount: {}", transaction.amount));
                            if ui.checkbox(&mut transaction.checked, "").clicked() {
                                self.transactions.insert(id, Transaction { ..transaction });
                            };
                        });
                    }
                });
            }

            if self.transactions.len() > 0 {
                let total = get_monthly_total(&self.transactions);
                ui.label(
                    RichText::new(format!("Total: {}", total)).font(FontId::proportional(28.0)),
                );
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

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
