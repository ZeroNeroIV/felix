//! Filename search — fast filename matching in current directory

use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn search_filenames(_dir: &Path, _query: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    // TODO: Use ignore crate for fast traversal + pattern matching
    todo!()
}
