//! Filename search — fast filename matching in current directory

use std::path::{Path, PathBuf};

pub fn search_filenames(dir: &Path, query: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    // TODO: Use ignore crate for fast traversal + pattern matching
    todo!()
}
