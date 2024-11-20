use std::fmt;
use std::string::String;

use crate::parser::token::{BlockType, Token};

#[derive(Debug, Clone)]
pub struct Block {
    block_type: BlockType,
    options: Vec<Token>,
}
impl Block {
    pub fn extract_blocks(content: &str, start: &str, end: &str) -> Vec<(Block, String)> {
        let mut blocks: Vec<(Block, String)> = Vec::new();
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
                //println!("{:?}", remainder);

                // 結果に追加
                buf.push((between, remainder));

                // 次の探索のために"end_here"の後に進める
                remaining = after_end;
            } else {
                break; // "end_here"が見つからない場合は終了
            }
        }
        //blocks
        //println!("buf block: {:?}", buf);
        for (between, remainder) in buf {
            if let Some(_remainder) = remainder {
                let b = between
                    .trim_start_matches(|c| c == ' ' || c == '\n')
                    .trim_end_matches(|c| c == ' ' || c == '\n')
                    .to_string();
                blocks.push(
                    (Block {
                    block_type: BlockType::MultiLine,
                    options: Vec::new(),
                    }, 
                     b.clone()
                    ))
            }
        }
        //println!("block extract: {:?}", blocks);
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
    fn new() -> Self;
    fn check_syntax_mode(&self, mode: String) -> Result<(), String>;
    fn get_value(&self, token: Token) -> Result<String, String>;
    //fn extract(content: Vec<Box<(Block, String)>>) -> Result<Vec<Box<Block>>, String>;
    fn extract(content: (Block, String)) -> Result<Self, String> where Self: Sized;
    fn convert_token(token_kind: &str, value: (String, Option<String>)) -> Result<Token, String>;
    fn make_token_package(raw: Vec<&str>) -> Result<Vec<Token>, String>;
    
}

#[derive(Debug)]
pub struct FileInfo {
    filename: Token,
    brief: Token,
    author: Token,
    date: Token,
}

/**
@Filename main.rs
@Brief テスト
@Author 俺
@Date 2024-11-20
**/

impl Extractor for FileInfo {
    fn new() -> Self {
        FileInfo {
            filename: Token::FileName("".to_string()),
            brief: Token::Brief("".to_string()),
            author: Token::Author("".to_string()),
            date: Token::Date("".to_string()),
        }
    }
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

    /*fn extract(content: Vec<Box<(Block, String)>>) -> Result<Vec<Box<Block>>, String> {
        println!("{:?}", content);
        let mut results = Vec::new();
        let content = Box::leak(content.into_boxed_slice());
        let mut buf;
        for raw in content {
            buf = raw.1.split_whitespace().collect::<Vec<&str>>();
            
            let ops = FileInfo::make_token_package(buf)?;
            
            results.push(Box::from(Block {
                block_type: BlockType::MultiLine,
                options: ops,
            }));
        }
        Ok(results)
    }*/
    fn extract(content: (Block, String)) -> Result<Self, String> {
        println!("{:?}", content);
        //let mut results = Vec::new();
        //let content = Box::leak(Box::<(Block, String)>::into_boxed_slice(content));
        //let mut buf;
        /*for raw in content {
            buf = raw.1.split_whitespace().collect::<Vec<&str>>();

            let ops = FileInfo::make_token_package(buf)?;

            results.push(Box::from(Block {
                block_type: BlockType::MultiLine,
                options: ops,
            }));
        }*/
        let buf = if let Ok(t) = FileInfo::make_token_package(content.1.split_whitespace().collect::<Vec<&str>>()) {t.clone()} else {Vec::new()};
        //let mut result = Extractor::new();]
        fn ext_ops<F>(vec: &[Token], condition: F) -> Option<&Token>
        where F: Fn(&Token) -> bool {
            vec.iter().find(|&i| condition(i))
        }
        
        Ok(Self {
            filename: if let Some(item) = 
                ext_ops(&*buf, |i| matches!(i, Token::FileName(_))) { item.clone() 
            } else { 
                Token::FileName("".to_string()) 
            },
            brief: if let Some(item) =
                ext_ops(&*buf, |i| matches!(i, Token::Brief(_))) { item.clone()
            } else {
                Token::FileName("".to_string())
            },
            author: if let Some(item) =
                ext_ops(&*buf, |i| matches!(i, Token::Author(_))) { item.clone()
            } else {
                Token::FileName("".to_string())
            },
            date: if let Some(item) =
                ext_ops(&*buf, |i| matches!(i, Token::Date(_))) { item.clone()
            } else {
                Token::FileName("".to_string())
            },
        })
        //Ok(results)
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
        let mut tokens: Vec<Token> = Vec::new();
        let mut index = 0;
        while index < raw.len().max(2) {
            let token_name = raw[index];
            let value = (raw[index + 1].to_string(), None);
            tokens.push(FileInfo::convert_token(token_name, value)?);
            index += 2;
        }
        Ok(tokens)
    }
}