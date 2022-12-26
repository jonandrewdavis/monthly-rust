#![allow(warnings, unused)]

use eframe::egui;
use error_chain::error_chain;
use serde_derive::Deserialize;
use std::collections::HashMap;

mod gui;
mod ledger;
use gui::MonthlyApp;

error_chain! {
    foreign_links {
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

// Result<(), csv::Error>
fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "monthly by andrew",
        options,
        Box::new(|_cc| Box::new(MonthlyApp::default())),
    );

    Ok(())
}
