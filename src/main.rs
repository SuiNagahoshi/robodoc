use robodoc::config::config;
//use robodoc::config::config::Config;
use robodoc::parser::parser;
use robodoc::parser::parser::Extractor;
use std::env;

//use robodoc::parser::parser_new::{generate_document, parse_comment_block};
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
        //let res: Vec<parser::Block> = parser::Block::split_blocks(comment);
        //println!("{:?}", res);
        let blocks = parser::Block::extract_blocks(comment, "/**", "**/");
        println!("main blocks {:?}", blocks);
        /*let _ex_block = if let Ok(v) = parser::FileInfo::extract(blocks) {
            for block in v {
                //let buf = block.options_raw
                println!("{:?}", parser::FileInfo::make_token_package(block.options_raw));
            }
        };*/
        let res = parser::FileInfo::extract(blocks);
        println!("extract result {:?}", res);
        if let Ok(r) = res {
            for i in r {
                println!("{}", i);
            }
        }
        /*std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024) // 32MBのスタックサイズに設定
            .spawn(|| {
                // extract関数をここで実行

            })
            .unwrap();
*/



        //let result = parser::FileInfo::make_token_package();
        /*
        let comment = r#"
        /**
         @FileName test.rs
         @Brief サンプル
         @Detail この関数はドキュメント生成ツールの例として作られました
         @Date x 2024
         @Author osaki
         **/
        "#;

            let file_info_block = parser::Block::split_blocks(comment);
            println!("{:?}", file_info_block);
        }
        */
    }
}
/*
use std::collections::HashMap;
use std::fmt;

// ドキュメントのタイプを表すEnum（ファイル、関数、クラスなど）
#[derive(Debug)]
enum DocType {
    File,
    Function,
    Class,
    Struct,
    Variable,
}

// ドキュメントのタイプごとに異なる出力を生成するトレイト
trait Documentable {
    fn to_html(&self) -> String;
    fn to_markdown(&self) -> String;
}

// パラメータの説明を保持する構造体
struct Param {
    name: String,
    description: String,
}

// 戻り値の説明を保持する構造体
struct Return {
    description: String,
}

// ドキュメントブロックの共通要素を保持する構造体
struct DocBlock {
    doc_type: DocType,
    brief: String,
    detail: Option<String>,
    params: Vec<Param>,
    ret: Option<Return>,
}

// ファイルや関数などに応じたドキュメント生成を実装
impl Documentable for DocBlock {
    fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<div class='doc-block'>");
        html.push_str(&format!("<h2>{:?} Documentation</h2>", self.doc_type));

        html.push_str(&format!("<p><strong>概要:</strong> {}</p>", self.brief));
        if let Some(detail) = &self.detail {
            html.push_str(&format!("<p><strong>詳細:</strong> {}</p>", detail));
        }

        if !self.params.is_empty() {
            html.push_str("<ul>");
            for param in &self.params {
                html.push_str(&format!(
                    "<li><strong>引数 {}:</strong> {}</li>",
                    param.name, param.description
                ));
            }
            html.push_str("</ul>");
        }

        if let Some(ret) = &self.ret {
            html.push_str(&format!("<p><strong>戻り値:</strong> {}</p>", ret.description));
        }

        html.push_str("</div>");
        html
    }

    fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str(&format!("## {:?} ドキュメント\n", self.doc_type));

        markdown.push_str(&format!("**概要:** {}\n", self.brief));
        if let Some(detail) = &self.detail {
            markdown.push_str(&format!("**詳細:** {}\n", detail));
        }

        if !self.params.is_empty() {
            markdown.push_str("\n**引数:**\n");
            for param in &self.params {
                markdown.push_str(&format!("- `{}`: {}\n", param.name, param.description));
            }
        }

        if let Some(ret) = &self.ret {
            markdown.push_str(&format!("\n**戻り値:** {}\n", ret.description));
        }

        markdown
    }
}

// コメントブロックからタグを抽出するパーサー関数
fn parse_comment_block(comment: &str) -> DocBlock {
    let mut doc_type = DocType::File;  // デフォルトはファイルタイプ
    let mut brief = String::new();
    let mut detail = None;
    let mut params = Vec::new();
    let mut ret = None;

    for line in comment.lines() {
        let line = line.trim();

        if line.starts_with("@Type") {
            if line.contains("Function") {
                doc_type = DocType::Function;
            } else if line.contains("Class") {
                doc_type = DocType::Class;
            } else if line.contains("Struct") {
                doc_type = DocType::Struct;
            } else if line.contains("Variable") {
                doc_type = DocType::Variable;
            }
        } else if line.starts_with("@Brief") {
            brief = line.replace("@Brief", "").trim().to_string();
        } else if line.starts_with("@Detail") {
            detail = Some(line.replace("@Detail", "").trim().to_string());
        } else if line.starts_with("@Param") {
            let param_line = line.replace("@Param", "").trim().to_string();
            let parts: Vec<&str> = param_line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let description = parts[1..].join(" ");
                params.push(Param { name, description });
            }
        } else if line.starts_with("@Return") {
            ret = Some(Return {
                description: line.replace("@Return", "").trim().to_string(),
            });
        }
    }

    DocBlock {
        doc_type,
        brief,
        detail,
        params,
        ret,
    }
}

fn main() {
    let comment = r#"
        /**
         * @Type Function
         * @Brief この関数は2つの数字を加算します。
         * @Detail 2つの整数を引数として受け取り、その合計を返します。
         * @Param a 最初の数字。
         * @Param b 2番目の数字。
         * @Return 2つの数字の合計。
         */
    "#;

    // コメントを解析
    let doc_block = parse_comment_block(comment);

    // HTMLとMarkdownの生成
    let html_output = doc_block.to_html();
    let markdown_output = doc_block.to_markdown();

    // コンソールに出力
    println!("HTML Output:\n{}", html_output);
    println!("Markdown Output:\n{}", markdown_output);
}*/
