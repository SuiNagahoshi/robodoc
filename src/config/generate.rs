pub mod generate {
    use serde::{Deserialize, Serialize};

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
    pub struct ConfigSet {
        common: CommonConfig,
        paths: PathConfig,
        output: OutputConfig,
    }
}