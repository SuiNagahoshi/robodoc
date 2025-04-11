use crate::config::structure::Config;
use std::fs;
use std::path;
impl Config {
    pub fn import<P: AsRef<path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::structure::{Config, Format, OutputLanguage, SourceLanguage};
    use std::path;

    #[test]
    fn test_parse_config() {
        let toml_str = r#"
[common]
license = "Apache-2.0"
project_name = "project"
date = "2025-04-11"
author = "user"
version = "0.1.0"
description = "This is a test"

[input]
path = "./src"
language = ["Rust"]

[output]
path = "./output"
language = ["Japanese"]
format = ["Html"]
"#;
        let config: Config = toml::from_str(toml_str).expect("Failed to parse toml");
        assert_eq!(config.common.license.as_deref(), Some("Apache-2.0"));
        assert_eq!(config.common.project_name, "project");
        assert_eq!(config.common.date, "2025-04-11");
        assert_eq!(config.common.author, "user");
        assert_eq!(config.common.version, "0.1.0");
        assert_eq!(config.common.description.as_deref(), Some("This is a test"));
        for lang in &config.input.language {
            assert!(matches!(lang, SourceLanguage::Rust));
        }
        assert_eq!(config.input.path, path::PathBuf::from("./src"));
        for lang in &config.output.language {
            assert!(matches!(
                lang,
                OutputLanguage::Japanese | OutputLanguage::English
            ));
        }
        assert_eq!(config.output.path, path::PathBuf::from("./output"));
        for format in &config.output.format {
            assert!(matches!(format, Format::Html));
        }
    }
}
