use chrono::{FixedOffset, Utc};
use std::fs;
use std::path;
use toml;
use crate::config::structure::{CommonConfig, Config, InputConfig, OutputConfig};

impl Config {
    pub fn new() -> Self {
        Self {
            common: CommonConfig {
                license: None,
                project_name: "".to_string(),
                date: Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap()).naive_local().date().to_string(),
                author: "".to_string(),
                version: "".to_string(),
                description: None,
            },
            input: InputConfig { 
                path: path::PathBuf::from("./src"),
                language: vec![] },
            output: OutputConfig {
                path: path::PathBuf::from("./docs"),
                language: vec![],
                format: vec![],
            },
        }
    }
    pub fn generate(&self, path: &path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_config() {
        let config = Config::new().generate(&path::PathBuf::from("config.toml"));
        let content = fs::read_to_string("config.toml").unwrap();
        let c = r#"[common]
project_name = ""
date = "2025-02-20"
author = ""
version = ""

[input]
path = "./src"
language = []

[output]
path = "./docs"
language = []
format = []
"#;
        assert_eq!(content, c.to_string());
        
    }
}