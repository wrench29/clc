use std::{path::PathBuf, str::FromStr};

#[macro_use]
extern crate serde_derive;

use analyzer::CodeAnalyzer;
use config::AnalyzerConfiguration;

mod analyzer;
mod config;

fn main() {
    let config =
        AnalyzerConfiguration::load_from_file().unwrap_or(AnalyzerConfiguration::default());
    let analyzer = CodeAnalyzer::new(config);
    let report = analyzer
        .analyze_dir(&PathBuf::from_str("../").unwrap())
        .unwrap();
    println!("{}", report);
}
