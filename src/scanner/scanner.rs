use crate::config::structure::SourceLanguage;

use std::path::PathBuf;
use walkdir::WalkDir;
use anyhow;
#[derive(Debug)]
pub struct SourceFile {
    pub file_ext: SourceLanguage,
    pub path: std::path::PathBuf,
    pub file_name: String,
    pub source: String,
}

pub fn scan_files(path: PathBuf) -> anyhow::Result<Vec<SourceFile>> {
    let mut result = Vec::new();
    let files = WalkDir::new(&path).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file());

    for file in files {
        let path = file.path().to_path_buf();
        if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
            let lang = SourceLanguage::from_extension(extension);
            if lang != SourceLanguage::Unknown {
                let content = std::fs::read_to_string(&path)?;
                result.push(SourceFile {
                    file_ext: lang,
                    path: path.clone(),
                    file_name: path.file_name().and_then(|s| s.to_str()).unwrap_or_default().to_string(),
                    source: content,
                })
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    // tests/test_scanner.rs
    use std::fs;
    use tempfile::tempdir;
    use super::*;

    #[test]
    fn test_find_source_files_rust_and_python() {
        let dir = tempdir().unwrap();
        let rust_file_path = dir.path().join("main.rs");
        let py_file_path = dir.path().join("script.py");

        fs::write(&rust_file_path, "fn main() {}\n").unwrap();
        fs::write(&py_file_path, "print(\"hello\")\n").unwrap();

        let paths = dir.path().to_string_lossy().to_string();
        let files = scan_files(paths.into()).unwrap();

        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f.file_ext == SourceLanguage::Rust));
        assert!(files.iter().any(|f| f.file_ext == SourceLanguage::Python));
    }

}