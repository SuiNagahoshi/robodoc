use regex::Regex;
use std::collections::VecDeque;

// InlineDoc構造体
#[derive(Debug)]
struct InlineDoc {
    doc: String,
    code: String,
}

// BlockNode構造体 (コードブロック)
#[derive(Debug)]
struct BlockNode {
    start_line: usize,
    end_line: usize,
    doc: Option<String>, // コメントブロックの内容
    children: Vec<BlockNode>, // 子ブロック
}

impl BlockNode {
    fn new(start_line: usize) -> Self {
        BlockNode {
            start_line,
            end_line: 0,
            doc: None,
            children: Vec::new(),
        }
    }

    fn set_end_line(&mut self, end_line: usize) {
        self.end_line = end_line;
    }

    fn add_child(&mut self, child: BlockNode) {
        self.children.push(child);
    }

    fn set_doc(&mut self, doc: String) {
        self.doc = Some(doc);
    }
}

// ドキュメント抽出関数
fn extract_docs(source: &str) -> Vec<InlineDoc> {
    let mut docs = Vec::new();
    let mut block_stack: VecDeque<BlockNode> = VecDeque::new();
    let re_block_start = Regex::new(r"\s*\{").unwrap(); // {の前後の空白を許容
    let re_block_end = Regex::new(r"\s*\}").unwrap();   // }の前後の空白を許容
    let re_doc_block = Regex::new(r"/\*\*([^\*]+)\*/").unwrap(); // /** コメントブロック */
    let re_inline_doc = Regex::new(r"///\s*(\w+)\s+(\w+)\s+(.*)").unwrap(); // /// インラインコメント

    let mut current_block: Option<BlockNode> = None;

    for (i, line) in source.lines().enumerate() {
        // コメントブロックを探す
        if let Some(caps) = re_doc_block.captures(line) {
            let doc = format!(
                "@type: {}\n@name: {}\n@description: {}",
                &caps[1], &caps[2], &caps[3]
            );
            if let Some(block) = current_block.as_mut() {
                block.set_doc(doc);
            }
        }

        // インラインコメントを探す
        if let Some(caps) = re_inline_doc.captures(line) {
            let code = format!("{};", line.trim()); // コード部分
            let doc = format!(
                "@type: {}\n@name: {}\n@description: {}",
                &caps[1], &caps[2], &caps[3]
            );
            docs.push(InlineDoc { doc, code });
        }

        // ブロック開始
        if re_block_start.is_match(line) {
            if let Some(parent_block) = current_block.take() {
                block_stack.push_back(parent_block);
            }
            current_block = Some(BlockNode::new(i + 1)); // 新しいブロックを開始
        }

        // ブロック終了
        if re_block_end.is_match(line) {
            if let Some(mut block) = current_block.take() {
                block.set_end_line(i + 1);
                if let Some(parent_block) = block_stack.pop_back() {
                    let mut parent_block = parent_block;
                    parent_block.add_child(block);
                    current_block = Some(parent_block);
                }
            }
        }
    }

    docs
}

// 実行テスト用コード
pub fn extract() {
    let source_code = r#"
mod analyzer;///mod analyzer import module
mod config;
mod scanner;
//use crate::analyzer::{output, python};
use crate::analyzer::rust;
use crate::config::structure::Config;
use crate::scanner::scanner::scan_files;
use std::env;
use std::path;

/**
@type function
@name main
@description main function
*/
fn main() {
    let foo = 42; ///var foo sample var

    /**
    @type block
    @name inner_check
    @description nested block
    */
    if foo == 42 {
        println!("Hello!"); ///call println say hello
    }
}
"#;

    let docs = extract_docs(source_code);

    for doc in docs {
        println!("DOC: {}", doc.doc);
        println!("CODE: {}", doc.code);
    }
}
