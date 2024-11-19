use robodoc::config::config;
use robodoc::parser::parser;
use robodoc::parser::parser::{Extractor};

use std::env;

fn main() {
    let comment = r#"
/**
@Filename main.rs
@Brief テスト
@Author 俺
@Date 2024
**/
Print("Hello World!")
/**
@Filename main.rs
@Brief テスト
@Author 俺
@Date 2024
**/
Print("Hello World!")

"#;
    println!("{}", comment);
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
        let config = config::Config::import_config("config.toml");
        match config {
            Ok(mut config) => {
                println!("設定内容: {:#?}", config);
                config = config;
            }
            Err(e) => eprintln!("設定ファイルの読み込みに失敗しました: {}", e),
        }
        let blocks = parser::Block::extract_blocks(comment, "/**", "**/");
        println!("main blocks {:?}", blocks);
        let res = parser::FileInfo::extract(blocks);
        println!("extract result {:?}", res);
        if let Ok(t) = res {
            for i in t {
                println!("{:?}", i);
            }
        }
    }
}