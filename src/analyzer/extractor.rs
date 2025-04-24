use regex::Regex;
use std::path::PathBuf;

use crate::scanner::scanner;

/// コメント種別
#[derive(Debug, PartialEq)]
pub enum CommentType {
    Block,   // /** ... */
    Line,    // /// ...
}

/// コメント情報
#[derive(Debug)]
pub struct Comment {
    pub comment_type: CommentType,
    pub content: String,
    pub file: PathBuf,
    pub line: usize,
}
pub fn extract_comments(source: &scanner::SourceFile) -> Vec<Comment> {
    let mut result = Vec::new();
    let text = &source.source;

    let re_block = Regex::new(r"/\*\*([\s\S]*?)\*/").unwrap();

    for cap in re_block.captures_iter(text) {
        let start_pos = text[..cap.get(0).unwrap().start()].matches('\n').count() + 1;
        result.push(Comment {
            comment_type: CommentType::Block,
            content: cap[1].trim().to_string(),
            file: source.path.clone(),
            line: start_pos,
        })
    }

    let re_line = Regex::new(r"(?m)^\s*///\s*(.*)$").unwrap();

    for (i, line) in text.lines().enumerate() {
        if let Some(cap) = re_line.captures(line) {
            result.push(Comment {
                comment_type: CommentType::Line,
                content: cap[1].trim().to_string(),
                file: source.path.clone(),
                line: i+1,
            })
        }
    }
    result
}

// 実行テスト用コード

#[cfg(test)]
mod tests {
    // tests/test_extractor.rs
    use crate::scanner::scanner::SourceFile;
    use crate::config::structure::SourceLanguage;
    //use rustdocgen::extractor::{extract_comments_from_source, CommentType};
    use super::*;

    #[test]
    fn test_extract_block_comment() {
        let file = SourceFile {
            file_ext: SourceLanguage::Rust,
            path: "test.rs".into(),
            file_name: "test.rs".to_string(),
            source: "/**\n@type function\n@name hello\n@description This is a test.\n*/\nfn hello() {}".to_string(),
        };

        let comments = extract_comments(&file);
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].comment_type, CommentType::Block);
        assert!(comments[0].content.contains("@type function"));
    }

    #[test]
    fn test_extract_line_comments() {
        let file = SourceFile {
            file_ext: SourceLanguage::Rust,
            path: "test.rs".into(),
            file_name: "test.rs".to_string(),
            source: "///type function name description\nfn main() {}".to_string(),
        };

        let comments = extract_comments(&file);
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].comment_type, CommentType::Line);
        assert!(comments[0].content.contains("function"));
    }

}