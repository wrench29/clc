use std::{env, path::PathBuf, process::exit, str::FromStr};

#[macro_use]
extern crate serde_derive;

use analyzer::CodeAnalyzer;
use config::AnalyzerConfiguration;

mod analyzer;
mod config;

fn main() {
    let mut code_path: String = String::from(".");
    if env::args().len() > 2 {
        eprintln!(
            "ERROR: Wrong arguments. Usage: {} [path_to_src]",
            env::args().nth(0).unwrap()
        );
        exit(1);
    } else if env::args().len() == 2 {
        code_path = env::args().last().unwrap();
    }
    let config_path = AnalyzerConfiguration::find_correct_config_file();
    let config = if let Some(config_path) = config_path {
        AnalyzerConfiguration::load_from_file(&config_path)
            .unwrap_or(AnalyzerConfiguration::default())
    } else {
        AnalyzerConfiguration::default()
    };
    let analyzer = CodeAnalyzer::new(config);
    let report = analyzer
        .analyze_dir(&PathBuf::from_str(&code_path).unwrap())
        .unwrap();
    println!("{}", report);
}
