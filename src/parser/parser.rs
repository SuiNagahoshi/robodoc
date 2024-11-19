use std::fmt;
use std::string::String;

use crate::parser::token::{BlockType, Token};

#[derive(Debug)]
pub struct Block {
    block_type: BlockType,
    options: Vec<Token>,
}
impl Block {
    pub fn extract_blocks(content: &str, start: &str, end: &str) -> Vec<Box<(Block, String)>> {
        let mut blocks: Vec<Box<(Block, String)>> = Vec::new();
        let mut buf = Vec::new();
        let mut remaining = content;
        while let Some(start_idx) = remaining.find(start) {
            let after_start = &remaining[start_idx + start.len()..];
            if let Some(end_idx) = after_start.find(end) {
                // "start_here"と"end_here"の間の部分を取得
                let between = &after_start[..end_idx];

                // "end_here"の後の部分（次の"start_here"まで）
                let after_end = &after_start[end_idx + end.len()..];
                let next_start_idx = after_end.find(start);

                // 次の"start_here"までの部分（次の"start_here"がない場合はNone）
                let remainder = if let Some(idx) = next_start_idx {
                    Some(&after_end[..idx])
                } else {
                    Some(after_end) // 最後の部分
                };

                // 結果に追加
                buf.push((between, remainder));

                // 次の探索のために"end_here"の後に進める
                remaining = after_end;
            } else {
                break; // "end_here"が見つからない場合は終了
            }
        }
        //blocks
        println!("buf block: {:?}", buf);
        for (between, remainder) in buf {
            if let Some(_remainder) = remainder {
                let b = between
                    .trim_start_matches(|c| c == ' ' || c == '\n')
                    .trim_end_matches(|c| c == ' ' || c == '\n')
                    .to_string();
                blocks.push(Box::from(
                    (Block {
                    block_type: BlockType::MultiLine,
                    options: Vec::new(),
                    }, 
                     b.clone()
                    )))
            }
        }
        println!("block extract: {:?}", blocks);
        blocks
    }
}

impl fmt::Display for Block {
    // This trait requires `fmt` with this exact signature.
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        // 厳密に最初の要素を、与えられた出力ストリーム `f` に書き込みます。
        // `fmt::Result`を返します。これはオペレーションが成功したか否か
        // を表します。
        // `write!`は`println!`に非常によく似た文法を使用していることに注目。
        write!(f, "{}", self)
    }
}
pub trait Extractor {
    fn check_syntax_mode(&self, mode: String) -> Result<(), String>;
    fn get_value(&self, token: Token) -> Result<String, String>;
    fn extract(content: Vec<Box<(Block, String)>>) -> Result<Vec<Box<Block>>, String>;
    fn convert_token(token_kind: &str, value: (String, Option<String>)) -> Result<Token, String>;
    fn make_token_package(raw: Vec<&str>) -> Result<Vec<Token>, String>;
}

pub struct FileInfo {
    tokens: Vec<Token>, //HashMap<u64, Token>,
}

impl Extractor for FileInfo {
    fn check_syntax_mode(&self, _mode: String) -> Result<(), String> { Ok(()) }

    fn get_value(&self, token: Token) -> Result<String, String> {
        match token {
            Token::FileName(t)
            | Token::Brief(t)
            | Token::Author(t)
            | Token::Date(t)
            | Token::Details(t) => Ok(t),
            _ => Err("Invalid token".to_string()),
        }
    }

    fn extract(content: Vec<Box<(Block, String)>>) -> Result<Vec<Box<Block>>, String> {
        println!("{:?}", content);
        let mut results = Vec::new();
        let content = Box::leak(content.into_boxed_slice());
        let mut buf;
        for raw in content {
            buf = raw.1.split_whitespace().collect::<Vec<&str>>();
            println!("{:?}", buf);
            
            let ops = FileInfo::make_token_package(buf)?;
            println!("{:?}", ops);
            
            results.push(Box::from(Block {
                block_type: BlockType::MultiLine,
                options: ops,
            }));
        }
        println!("{:?}", results);
        Ok(results)
    }
    fn convert_token(token_kind: &str, value: (String, Option<String>)) -> Result<Token, String> {
        match token_kind {
            "@Filename" => Ok(Token::FileName(value.0.to_string())),
            "@Brief" => Ok(Token::Brief(value.0.to_string())),
            "@Author" => Ok(Token::Author(value.0.to_string())),
            "@Date" => Ok(Token::Date(value.0.to_string())),
            "@Details" => Ok(Token::Details(value.0.to_string())),
            _ => Err("Invalid token".to_string()),
        }
    }

    fn make_token_package(raw: Vec<&str>) -> Result<Vec<Token>, String> {
        println!("raw {:?}", raw);
        let mut tokens: Vec<Token> = Vec::new();
        let mut index = 0;
        while index < raw.len().max(2) {
            let token_name = raw[index];
            let value = (raw[index + 1].to_string(), None);
            println!("token: {:?}", token_name);
            println!("value: {:?}", value);
            tokens.push(FileInfo::convert_token(token_name, value)?);
            index += 2;
        }
        println!("tokens: {:?}", tokens);
        Ok(tokens)
    }
}