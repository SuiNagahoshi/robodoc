/*use crate::parser::token::Token;

pub struct TokenSet {
    token_kind: Token,
    token: String,
}

pub trait Tokenizer {
    fn tokenize(&self, split: (&str, &str), ) -> Vec<TokenSet>;
}*/
use std::fmt;
//use std::collections::HashMap;
use crate::parser::token::{BlockType, Token};
use std::string::String;

#[derive(Debug)]
pub struct Block {
    //start: usize,
    //end: usize,
    pub block_type: BlockType,
    //pub options_raw: Box<String>,
    pub options: Vec<Token>,
    //pub source: Box<String>,
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
            /*println!("Between: {}", between);
            if let Some(_remainder) = _remainder {
                println!("Remainder: {}", _remainder);
            }*/
            if let Some(_remainder) = remainder {
                let b = between
                    .trim_start_matches(|c| c == ' ' || c == '\n')
                    .trim_end_matches(|c| c == ' ' || c == '\n')
                    .to_string();
                blocks.push(Box::from(
                    (Block {
                    block_type: BlockType::MultiLine,
                    //options_raw: Box::from(b.clone()),
                    options: Vec::new(),
                    //source: Box::from(b.clone()), 
                    }, 
                     b.clone()
                    )))
            }
        }
        println!("block extract: {:?}", blocks);
        blocks
    }
    /*fn extract_ops(content: &str, start: &str, end: &str) -> Vec<String> {
        let mut buf = Vec::new();
        let mut remaining = content;
        /*if Some(t) = remaining.chars().last() {
            remaining
        }*/
        println!("extract remaining: {:?}", remaining);
        while let Some(start_index) = remaining.find(start) {
            let after_start = &remaining[start_index + start.len()..];
            if let Some(end_index) = after_start.find(end) {
                buf.push(after_start[..end_index].to_string());
                remaining = &after_start[end_index + end.len()..];
            } else {
                break;
            }
        }
        let mut results: Vec<String> = Vec::new();
        for mut i in 0..buf.len() {
            //buf[i].retain(|c| c != '\n');
            results = buf[i].split_whitespace().map(|s| s.to_string()).collect();
        }
        println!("extract results: {:?}", results);
        results
    }
    /*fn set_token(content: Vec<String>) -> Vec<Token> {
        let mut buf = content;
        let mut token: Token;
        let mut results: Vec<Token> = Vec::new();
        let mut index = 0;
        while index < buf.len().max(2) {

            //println!("block: {:?}", blocks);
            index += 2;
        }
        results
    }*/
    pub fn split_blocks(content: &str) -> Vec<Block> {
        let mut blocks: Vec<Block> = Vec::new();

        let mut buf: Vec<String> = Self::extract_ops(content, "/**", "**/
");
        /*let mut block: Block = Block {
            block_type: BlockType::MultiLine,
            options: "".to_string(),
            source: "".to_string(),
        };*/
        println!("block buf: {:?}", buf);
        //let test = buf[0].split_whitespace().collect();
        //let range = (buf.len() / 2).max(1);
        //println!("block range: {:?}", range);
        let mut index = 0;
        while index < buf.len().max(2) {
            blocks.push(Block {
                block_type: BlockType::MultiLine,
                options_row: buf[index].clone(),
                options: Vec::new(),
                source: buf[index + 1].clone(),
            });
            println!("block: {:?}", blocks);
            index += 2;
        }
        /*for i in 0..range {
            //block.options = buf[i].clone();
            //block.source = buf[i + 1].clone();
            blocks.push(Block {
                block_type: BlockType::MultiLine,
                options: buf[i].clone(),
                source: buf[i + 1].clone(),
            });
            println!("block: {:?}", blocks);
        }*/

        blocks
    }*/
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

    //fn set_hash(&self,value: Token);
    //fn get_token_value(&self, token: Token) -> Result<String, String>;
    //fn get_hash_value(&self, key: u64) -> Result<Token, String>;
    //fn get_value(&self, key: Option<u64>, value: Option<Token>) -> Option<String>;
    //fn set_vec(&mut self, value: Token);
    fn get_value(&self, token: Token) -> Result<String, String>;
    fn extract(content: Vec<Box<(Block, String)>>) -> Result<Vec<Box<Block>>, String>;
    fn convert_token(token_kind: &str, value: (String, Option<String>)) -> Result<Token, String>;
    fn make_token_package(raw: Vec<&str>) -> Result<Vec<Token>, String>;
}

pub struct FileInfo {
    tokens: Vec<Token>, //HashMap<u64, Token>,
}

impl Extractor for FileInfo {
    fn check_syntax_mode(&self, _mode: String) -> Result<(), String> {
        Ok(())
    }

    /*fn set_vec(&mut self, value: Token) {
        self.tokens.push(value);
        /*if !&self.tokens.is_empty() {
            let key = &self.tokens.len();
            self.tokens.insert((*key as u64) + 1, value);
        } else {
            self.tokens.insert(1, value);
        }*/
    }*/

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
    /*fn get_hash_value(&self, key: u64) -> Result<Token, String> {
    match *self.tokens.get(&key) {
        Some(t) => {
            match t {
                Token::FileName(_) |
                Token::Brief(_) |
                Token::Author(_) |
                Token::Date(_) |
                Token::Details(_) => {Ok(t)}
                _ => {
                    Err("Invalid keys".to_string())
                }
            }
        },
        None => {
            Err("Invalid keys".to_string())
        }}
    }*/

    fn extract(content: Vec<Box<(Block, String)>>) -> Result<Vec<Box<Block>>, String> {
        /*let row = &content[1];
        let mut buf: Vec<&str> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();
        for line in row.options_row.lines() {
            buf = line.splitn(2, ' ').collect();
            for i in buf {
                match i {
                    "@Filename" => tokens.push(Token::FileName(i.to_string())),
                    "@Brief" => tokens.push(Token::Brief(i.to_string())),
                    "@Author" => tokens.push(Token::Author(i.to_string())),
                    "@Date" => tokens.push(Token::Date(i.to_string())),
                    "@Details" => tokens.push(Token::Details(i.to_string())),
                    _ => return Err("Invalid token".to_string())
                }
            }
        }
        Ok(tokens)*/
        println!("{:?}", content);
        let mut results = Vec::new();
        let content = Box::leak(content.into_boxed_slice());
        let mut buf;
        for raw in content {
            //let raw_buf = raw.options_raw.clone();
            //let source_buf = raw.source.clone();
            buf = raw.1.split_whitespace().collect::<Vec<&str>>();
            //let len = .len().max(2);
            println!("{:?}", buf);
            //let mut index = 0;
            let ops = FileInfo::make_token_package(buf)?;
            println!("{:?}", ops);
            results.push(Box::from(Block {
                block_type: BlockType::MultiLine,
                //options_raw: raw.options_raw.clone(),
                options: ops,
                //source: Box::from(raw.1.clone()),
            }));
            /*while index < len {
                let mut ops = super::parser::FileInfo::make_token_package(buf)?;
                println!("{:?}", ops);
                results.push(Block {
                    block_type: BlockType::MultiLine,
                    options_raw: buf[index].clone().parse().unwrap(),
                    options: ops,
                    source: buf[index + 1].clone().parse().unwrap(),
                });
                println!("block: {:?}", results);
                index += 2;
            }*/
        }
        println!("{:?}", results);
        Ok(results)
    }
    fn convert_token(token_kind: &str, value: (String, Option<String>)) -> Result<Token, String> {
        //let mut token;
        match token_kind {
            "@Filename" => Ok(Token::FileName(value.0.to_string())),
            "@Brief" => Ok(Token::Brief(value.0.to_string())),
            "@Author" => Ok(Token::Author(value.0.to_string())),
            "@Date" => Ok(Token::Date(value.0.to_string())),
            "@Details" => Ok(Token::Details(value.0.to_string())),
            _ => Err("Invalid token".to_string()),
        }
        //Ok(token)
    }

    fn make_token_package(raw: Vec<&str>) -> Result<Vec<Token>, String> {
        println!("raw {:?}", raw);
        //let buf = raw.split_whitespace().collect::<Vec<&str>>();
        //println!("make token {:?}", buf);
        let mut tokens: Vec<Token> = Vec::new();
        /*for c in buf {
            if c.as_str().chars().nth(0).unwrap() = "@" {}
        }*/
        let mut index = 0;
        while index < raw.len().max(2) {
            let token_name = raw[index];
            let value = (raw[index + 1].to_string(), None);
            /*let first_char = buf[index].chars().nth(0);
            if let Some('@') = first_char {
                token_name = buf[index];
            } else { //if buf[index].chars().nth(0).unwrap() != '@' && buf[index+1].chars().nth(0).unwrap() = '@' {
                value = (buf[index].to_string(), None);
            }*/

            println!("token: {:?}", token_name);
            println!("value: {:?}", value);
            //let token_name = if buf[index].as_str().chars().nth(0).unwrap() = "@" { buf[index]} else {index += 1 };
            //let value: (String, Option<String>) = if buf[index].as_str().chars().nth(0).unwrap() != "@" && buf[index + 1].as_str().chars().nth(0).unwrap() = "@" { (buf[index].to_string(), None)} else {index += 1};
            tokens.push(FileInfo::convert_token(token_name, value)?);
            index += 2;
        }
        println!("tokens: {:?}", tokens);
        Ok(tokens)
    }
}
/*
pub trait Extractor {
    fn check_syntax_mode(&self, mode: String) -> Result<(), String>;
    fn set_hash(&self,key: Token , value: (Option<String>, Option<String>));
    fn get_value(&self, key: Token) -> (Option<String>, Option<String>);
    fn extract(&self, content: Vec<Block>) -> HashMap<Token, Option<String>>;
}

struct FileInfo {
    tokens: HashMap<Token, Option<String>>//(Option<String>, Option<String>)>
}

impl Extractor for FileInfo {
    fn check_syntax_mode(&self, mode: &str) -> Result<(), String> {
        if mode == "f" {
            for i in self.tokens {
                /*if i != Token::FileName | Token::Brief{
                    return Err("Error: An invalid parameter has been set.".to_string())
                }*/
                match i.keys() {
                    Token::FileName |
                    Token::Brief |
                    Token::Author |
                    Token::Date |
                    Token::Details => {}
                    _ => {return Err("test".to_string())}
                }
            }
        }
        Ok(())
    }
    fn set_hash(&mut self, key: Token, value: (Option<String>, Option<String>)) {
        self.tokens.insert(key, value.1);
    }

    fn get_value(&self, key: Token) -> (Option<String>, Option<String>) {
        *self.tokens.get(&key)
    }

    /*fn extract(&self, content: Vec<Block>) -> HashMap<Token, Option<String>> {
        let mut result: HashMap<Token, Option<String>> = HashMap::new();
        let splited: Vec<&str> = content.split('\n').collect();
        for i in splited {

        }
    }*/
    fn extract(&self, content: Vec<Block>) -> HashMap<Token, Option<String>> {
        let mut result: HashMap<Token, Option<String>> = HashMap::new();
    }
}
*/
