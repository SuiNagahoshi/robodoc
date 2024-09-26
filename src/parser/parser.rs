/*use crate::parser::token::Token;

pub struct TokenSet {
    token_kind: Token,
    token: String,
}

pub trait Tokenizer {
    fn tokenize(&self, split: (&str, &str), ) -> Vec<TokenSet>;
}*/
//use std::collections::HashMap;
use std::string::String;
use crate::parser::token::{BlockType, Token};

#[derive(Debug)]
pub struct Block {
    //start: usize,
    //end: usize,
    block_type: BlockType,
    options: String,
    source: String,
}

impl Block {
    fn extract_ops(content: &str, start: &str, end: &str) -> Vec<String> {
        println!("{:?}", content);
        let mut results = Vec::new();
        let mut remaining = content;
        while let Some(start_index) = remaining.find(start) {
            let after_start = &remaining[start_index + start.len()..];
            if let Some(end_index) = after_start.find(end) {
                results.push(after_start[..end_index].to_string());
                remaining = &after_start[end_index + end.len()..];
            } else {
                break;
            }
        }
        println!("{:?}", results);
        results
    }
    pub fn split_blocks(content: &str) -> Vec<Block> {
        let mut blocks: Vec<Block> = Vec::new();
        let mut buf: Vec<String> = Self::extract_ops(content, "/**", "**/");
        /*let mut block: Block = Block {
            block_type: BlockType::MultiLine,
            options: "".to_string(),
            source: "".to_string(),
        };*/
        println!("{:?}", buf);
        for i in 0..(buf.len() / 2) {
            //block.options = buf[i].clone();
            //block.source = buf[i + 1].clone();
            blocks.push(Block {
                block_type: BlockType::MultiLine,
                options: buf[i].clone(),
                source: buf[i + 1].clone(),
            });
        }
        println!("{:?}", blocks);
        blocks
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
    fn extract(&self, content: Vec<Block>) -> Result<Vec<Token>, String>;



}

struct FileInfo {
    tokens: Vec<Token>//HashMap<u64, Token>,
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
            Token::FileName(t)|
            Token::Brief(t)|
            Token::Author(t)|
            Token::Date(t)|
            Token::Details(t) => {
                Ok(t)
            }
            _ => {
                Err("Invalid token".to_string())
            }
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

    fn extract(&self, content: Vec<Block>) -> Result<Vec<Token>, String> {
        let row = &content[1];
        let mut buf: Vec<&str> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();
        for line in row.options.lines() {
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