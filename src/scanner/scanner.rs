use crate::config::structure::SourceLanguage;

use std::path::PathBuf;
use walkdir::WalkDir;
#[derive(Debug)]
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
}
pub fn scan_files(path: PathBuf) /* -> Vec<SourceFile>*/
{
    let mut list: Vec<SourceFile> = Vec::new();
    let allow_extensions = vec!["rs".to_string(), "txt".to_string()];
    let entries: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext_str| allow_extensions.contains(&ext_str.to_lowercase()))
                    .unwrap_or(false)
        })
        .collect();
    entries.iter().for_each(|entry| {
        let path = entry.path();
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("None");

        println!(
            "Path: {}\n  File: {}\n  Ext: {}\n",
            path.display(),
            file_name,
            extension
        );
        list.push(SourceFile {
            file_ext: SourceFile::convert_to_lang_type(extension),
            path: path.to_owned(),
            file_name: file_name.to_owned(),
        });
    });
    println!("{:?}", list);
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
