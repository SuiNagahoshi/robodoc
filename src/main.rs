mod config;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Argument error! Please provide one argument.\n'init', 'generate'");
    }
    if args[1] == "init" {
        println!("Initializing...");
    } else if args[1] == "generate" {
        println!("Generating...");
    } else {
        eprintln!("Argument error! Please provide one argument.\n'init', 'generate'");
    }
}
