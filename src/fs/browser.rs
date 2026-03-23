//! File system browser — navigation, listing, metadata

use std::path::{Path, PathBuf};

pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<std::time::SystemTime>,
}

pub fn list_directory(path: &Path) -> Result<Vec<FileEntry>, std::io::Error> {
    // TODO: Use ignore crate for fast directory walking
    todo!()
}
