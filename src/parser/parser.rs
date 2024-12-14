use std::fmt;
use std::string::String;
use crate::parser::token::{LineType, FileInfoToken, BlockType};

#[derive(Debug, Clone)]
pub struct Block {
    line_type: LineType,
    content: Box<BlockType>,
}
impl Block {
    pub fn extract_blocks(content: &str, start: &str, end: &str) -> Vec<Block> {
        let mut blocks: Vec<Block> = Vec::new();
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
                let line:LineType;
                if let Some(_space) = b.find('\n') {
                    line = LineType::MultiLine;
                } else {
                    line = LineType::SingleLine;
                }
                let content = parse_block(BlockType::Raw(b.clone()));
                blocks.push(
                    Block {
                    line_type: line,
                    content: Box::from(content),
                    })
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
    fn get_value(&self, token: FileInfoToken) -> Result<String, String>;
    //fn extract(content: Vec<Box<(Block, String)>>) -> Result<Vec<Box<Block>>, String>;
    fn extract(content: String) -> Result<Self, String> where Self: Sized;
    fn convert_token(token_kind: &str, value: (String, Option<String>)) -> Result<FileInfoToken, String>;
    fn make_token_package(raw: Vec<&str>) -> Result<Vec<FileInfoToken>, String>;
    
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    filename: FileInfoToken,
    description: FileInfoToken,
    author: FileInfoToken,
    date: FileInfoToken,
}

/**
@Filename main.rs
@Description テスト
@Author 俺
@Date 2024-11-20
**/

impl Extractor for FileInfo {
    fn new() -> Self {
        FileInfo {
            filename: FileInfoToken::FileName("".to_string()),
            description: FileInfoToken::Description("".to_string()),
            author: FileInfoToken::Author("".to_string()),
            date: FileInfoToken::Date("".to_string()),
        }
    }
    fn check_syntax_mode(&self, _mode: String) -> Result<(), String> { Ok(()) }

    fn get_value(&self, token: FileInfoToken) -> Result<String, String> {
        match token {
            FileInfoToken::FileName(t)
            | FileInfoToken::Description(t)
            | FileInfoToken::Author(t)
            | FileInfoToken::Date(t) => Ok(t),
        }
    }
    fn extract(content: String) -> Result<Self, String> {
        println!("{:?}", content);
        let buf = if let Ok(t) = FileInfo::make_token_package(content.split_whitespace().collect::<Vec<&str>>()) {t.clone()} else {Vec::new()};
        //let mut result = Extractor::new();]
        fn ext_ops<F>(vec: &[FileInfoToken], condition: F) -> Option<&FileInfoToken>
        where F: Fn(&FileInfoToken) -> bool {
            vec.iter().find(|&i| condition(i))
        }
        
        Ok(Self {
            filename: if let Some(item) = 
                ext_ops(&*buf, |i| matches!(i, FileInfoToken::FileName(_))) { item.clone()
            } else { 
                FileInfoToken::FileName("".to_string())
            },
            description: if let Some(item) =
                ext_ops(&*buf, |i| matches!(i, FileInfoToken::Description(_))) { item.clone()
            } else {
                FileInfoToken::FileName("".to_string())
            },
            author: if let Some(item) =
                ext_ops(&*buf, |i| matches!(i, FileInfoToken::Author(_))) { item.clone()
            } else {
                FileInfoToken::FileName("".to_string())
            },
            date: if let Some(item) =
                ext_ops(&*buf, |i| matches!(i, FileInfoToken::Date(_))) { item.clone()
            } else {
                FileInfoToken::FileName("".to_string())
            },
        })
        //Ok(results)
    }
    fn convert_token(token_kind: &str, value: (String, Option<String>)) -> Result<FileInfoToken, String> {
        match token_kind {
            "@Filename" => Ok(FileInfoToken::FileName(value.0.to_string())),
            "@Description" => Ok(FileInfoToken::Description(value.0.to_string())),
            "@Author" => Ok(FileInfoToken::Author(value.0.to_string())),
            "@Date" => Ok(FileInfoToken::Date(value.0.to_string())),
            _ => Err("Invalid token".to_string()),
        }
    }

    fn make_token_package(raw: Vec<&str>) -> Result<Vec<FileInfoToken>, String> {
        let mut tokens: Vec<FileInfoToken> = Vec::new();
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

pub fn parse_block(content: BlockType) -> BlockType {
    let mut res = content.clone();
    let patterns = vec!["@Filename".to_string()];
    let raw = match content {
        BlockType::Raw(t) => t,
        _ => return content,
    };

    for pattern in patterns {
        if raw.contains(&pattern) {
            match pattern.as_str() {
                "@Filename" => {
                    let ret = FileInfo::extract(raw.clone());
                    if let Ok(r) = ret {
                        res = BlockType::FileInfo(r);
                    }
                }
                _ => {
                    res = BlockType::Raw(raw.clone())
                },
            }
        }
    }
    res
}