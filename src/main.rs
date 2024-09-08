use robodoc::config::config;
use std::env;
use robodoc::config::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        let config = config::Config::new();
        match config.to_toml() {
            Ok(toml_string) => println!("{}", toml_string),
            Err(e) => println!("エラー{}", e),
        }
        if let Err(e) = config.generate("config.toml") {
            eprintln!("エラー{}", e);
        }
    } else if args[1] == "generate" {
        let mut config = Config::import_config("config.toml");
        match config {
            Ok(mut config) => {
                println!("設定内容: {:#?}", config);
                config = config;
            }
            Err(e) => eprintln!("設定ファイルの読み込みに失敗しました: {}", e),
        }
    }
}