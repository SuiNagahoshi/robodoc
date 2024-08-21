use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonConfig {
    #[serde(default = "default_project_name")]
    project_name: String,
    #[serde(default = "default_language")]
    language: String,
    #[serde(default = "default_entry_file")]
    entry_file: String,
}

fn default_project_name() -> String {
    "".to_string()
}
fn default_entry_file() -> String {
    "".to_string()
}
fn default_language() -> String {
    "".to_string()
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
pub struct OutputConfig {
    #[serde(default = "default_output_format")]
    format: String,
    #[serde(default = "default_output_language")]
    document_language: String,
    #[serde(default = "default_source_include")]
    source_include: bool,
}

fn default_output_format() -> String {
    "html".to_string()
}
fn default_output_language() -> String {
    "japanese".to_string()
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
                project_name: default_project_name(),
                language: default_language(),
                entry_file: default_entry_file(),
            },
            paths: PathConfig {
                output_path: default_output_path(),
                input_path: default_input_path()
            },
            output: OutputConfig {
                format: default_output_format(),
                document_language: default_output_language(),
                source_include: default_source_include(),
            },
        }
    }
}