mod config;
mod scanner;
use crate::config::structure::Config;

use std::env;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Argument error! Please provide one argument.\n'init', 'generate'");
    }
    if args[1] == "init" {
        println!("Initializing...");
        let config = Config::new();
        let _ = config.generate(&path::PathBuf::from("config.toml"));
    } else if args[1] == "generate" {
        println!("Generating...");
        let config = Config::import(path::PathBuf::from("config.toml"));
        println!("{:?}", config);

    } else {
        eprintln!("Argument error! Please provide one argument.\n'init', 'generate'");
    }
}
