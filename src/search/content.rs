//! Content search — grep-like search using ripgrep's engine

use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub struct SearchMatch {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
}

#[allow(dead_code)]
pub fn search_content(_dir: &Path, _query: &str) -> Result<Vec<SearchMatch>, std::io::Error> {
    // TODO: Use grep-searcher or ignore crate for fast content search
    todo!()
}
