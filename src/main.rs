use std::{path::PathBuf, str::FromStr};

use analyzer::CodeAnalyzer;
use config::AnalyzerConfiguration;

mod analyzer;
mod config;

fn main() {
    let config = AnalyzerConfiguration::default();
    let analyzer = CodeAnalyzer::new(config);
    let report = analyzer
        .analyze_dir(&PathBuf::from_str(".").unwrap())
        .unwrap();
    println!("{}", report);
}
