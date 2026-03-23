//! Content search — grep-like search using ripgrep's engine

use std::path::{Path, PathBuf};

pub struct SearchMatch {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
}

pub fn search_content(dir: &Path, query: &str) -> Result<Vec<SearchMatch>, std::io::Error> {
    // TODO: Use grep-searcher or ignore crate for fast content search
    todo!()
}
