use crate::config::structure::SourceLanguage;
pub struct SourceFile {
    file_ext: SourceLanguage,
    path: std::path::PathBuf,
    file_name: String,
}
impl SourceFile {
    pub fn convert_to_lang_type(ext: &str) -> SourceLanguage {
        match ext {
            "rs" => SourceLanguage::Rust,
            _ => SourceLanguage::Text,
        }
    }

    pub fn convert_to_extension(lang_type: SourceLanguage) -> &'static str {
        match lang_type {
            SourceLanguage::Rust => "rs",
            _ => "txt",
        }
    }
    /*pub fn scan_dirs(path: &path::PathBuf) -> Result<Vec<SourceFile>, String> {

    }*/
}

#[cfg(test)]
mod tests {
    use crate::config::structure::SourceLanguage;
    use crate::scanner::scanner::SourceFile;
    #[test]
    fn test_convert_to_lang_type() {
        assert_eq!(SourceLanguage::Rust, SourceFile::convert_to_lang_type("rs"));
    }

    #[test]
    fn test_convert_to_extension() {
        assert_eq!("rs", SourceFile::convert_to_extension(SourceLanguage::Rust));
    }
}