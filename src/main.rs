mod analyzer; ///mod analyzer import module
mod config;
mod scanner;
//use crate::analyzer::{output, python};
use crate::analyzer::rust;
use crate::config::structure::Config;
use crate::scanner::scanner::scan_files;
use std::env;
use std::path;

/**
@type function
@name main
@description main function
*/
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Argument error! Please provide one argument.\n'init', 'generate'")
    }
    /**
    @type if
    @description read len
    */
    if args[1] == "init" {
        println!("Initializing...");
        let config = Config::new();///var config create config
        let _ = config.generate(&path::PathBuf::from("config.toml"));
    } else if args[1] == "generate" {
        println!("Generating...");
        let config = Config::import(path::PathBuf::from("config.toml"));
        println!("{:?}", config);
        //println!("{:?}", config.unwrap().input.path);
        scan_files(config.unwrap().input.path);

        rust::extract();
    } else {
        eprintln!("Argument error! Please provide one argument.\n'init', 'generate'");
    }
}
