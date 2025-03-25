use std::path::PathBuf;
use serde::{Deserialize, Serialize};
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
pub enum SourceLanguage {
    Rust,
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
    HTML
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputConfig {
    pub path: PathBuf,
    pub language: Vec<OutputLanguage>,
    pub format: Vec<Format>,
}
