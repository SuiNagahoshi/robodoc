use robodoc::config::generate;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        let config = generate::Config::new();
        match config.to_toml() {
            Ok(toml_string) => println!("{}", toml_string),
            Err(e) => println!("エラー{}", e),
        }
        if let Err(e) = config.generate("config.toml") {
            eprintln!("エラー{}", e);
        }
    } else if args[1] == "generate" {
        println!("Generation...");
    }
}
