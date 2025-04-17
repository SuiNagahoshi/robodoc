use serde::{Deserialize, Serialize};
use std::path::PathBuf;
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub common: CommonConfig,
    pub input: InputConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommonConfig {
    pub license: Option<String>,
    pub project_name: String,
    pub date: String,
    pub author: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
//#[serde(rename_all = "PascalCase")]
#[derive(PartialEq)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceLanguage {
    Rust,
    Cpp,
    Python,
    Java,
    JavaScript,
    Arduino,
    Unknown,
}

impl SourceLanguage {
    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "rs" => SourceLanguage::Rust,
            "cpp" | "hpp" | "c" | "h" => SourceLanguage::Cpp,
            "py" => SourceLanguage::Python,
            "java" => SourceLanguage::Java,
            "js" | "ts" => SourceLanguage::JavaScript,
            "ino" => SourceLanguage::Unknown,
            _ => SourceLanguage::Unknown,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputConfig {
    pub path: PathBuf,
    pub language: Vec<SourceLanguage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OutputLanguage {
    Japanese,
    English,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Format {
    Html,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputConfig {
    pub path: PathBuf,
    pub language: Vec<OutputLanguage>,
    pub format: Vec<Format>,
}
