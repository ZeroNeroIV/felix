//! Virtual file system for browsing inside archives

use std::path::Path;

pub fn is_archive(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some("zip") | Some("tar") | Some("gz") | Some("7z") => true,
        _ => false,
    }
}

pub fn list_archive(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // TODO: Detect format and list contents
    todo!()
}
