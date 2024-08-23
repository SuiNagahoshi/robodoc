use serde::{Deserialize, Serialize};
use std::fs;
use chrono::{Utc, FixedOffset};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonConfig {
    #[serde(default = "default_empty")]
    project_name: String,
    #[serde(default = "default_empty")]
    author_name: String,
    #[serde(default = "default_empty")]
    version: String,
    #[serde(default = "default_date")]
    date: String,
    #[serde(default = "default_empty")]
    language: String,
    #[serde(default = "default_empty")]
    entry_file: String,
}
fn default_empty() -> String {
    "".to_string()
}
fn default_date() -> String {
    Utc::now()
        .with_timezone(
            &FixedOffset::east_opt(9 * 3600).unwrap())
        .naive_local()
        .date()
        .to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathConfig {
    #[serde(default = "default_output_path")]
    output_path: String,
    #[serde(default = "default_input_path")]
    input_path: String,
}
fn default_output_path() -> String {
    "docs/".to_string()
}
fn default_input_path() -> String {
    "src/".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Format {
    HTML,
    PDf,
    MARKDOWN,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Language {
    JAPANESE,
    ENGLISH,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputConfig {
    #[serde(default = "default_output_format")]
    format: Format,
    #[serde(default = "default_output_language")]
    document_language: Language,
    #[serde(default = "default_source_include")]
    source_include: bool,
}
fn default_output_format() -> Format {
    Format::HTML
}
fn default_output_language() -> Language {
    Language::JAPANESE
}
fn default_source_include() -> bool {
    true
}
//TODO input_configuration

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    common: CommonConfig,
    path: PathConfig,
    output: OutputConfig,
}
impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(&self)
    }

    pub fn generate(&self, path: &str) -> Result<(), std::io::Error> {
        let toml_string: String = self.to_toml().expect("error");
        fs::write(path, toml_string)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            common: CommonConfig {
                project_name: default_empty(),
                author_name: default_empty(),
                version: default_empty(),
                date: default_date(),
                language: default_empty(),
                entry_file: default_empty(),
            },
            path: PathConfig {
                output_path: default_output_path(),
                input_path: default_input_path(),
            },
            output: OutputConfig {
                format: default_output_format(),
                document_language: default_output_language(),
                source_include: default_source_include(),
            },
        }
    }
}