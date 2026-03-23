//! Virtual file system for browsing inside archives

use std::path::Path;

#[allow(dead_code)]
pub fn is_archive(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("zip") | Some("tar") | Some("gz") | Some("7z")
    )
}

#[allow(dead_code)]
pub fn list_archive(_path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // TODO: Detect format and list contents
    todo!()
}
