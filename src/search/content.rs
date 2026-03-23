//! Content search — grep-like search using ripgrep's engine

use grep_regex::RegexMatcher;
use grep_searcher::sinks::Lossy;
use grep_searcher::Searcher;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// A single search match result.
#[allow(dead_code)]
pub struct SearchMatch {
    pub path: PathBuf,
    pub line_number: u64,
    pub line: String,
}

/// Search for content matching the query string across files in a directory.
/// Case-insensitive by default. Returns up to `limit` results (0 = unlimited).
#[allow(dead_code)]
pub fn search_content(
    dir: &Path,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchMatch>, Box<dyn std::error::Error>> {
    if query.is_empty() {
        return Ok(Vec::new());
    }

    // Build a case-insensitive regex from the literal query
    let pattern = format!("(?i){}", regex::escape(query));
    let matcher = RegexMatcher::new(&pattern)?;

    let mut results = Vec::new();
    let mut searcher = Searcher::new();

    let walker = WalkBuilder::new(dir)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    for entry in walker.flatten() {
        if limit > 0 && results.len() >= limit {
            break;
        }

        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if is_binary_extension(path) {
            continue;
        }

        let path_buf = path.to_path_buf();
        let search_result = searcher.search_path(
            &matcher,
            path,
            Lossy(|lnum, line| {
                results.push(SearchMatch {
                    path: path_buf.clone(),
                    line_number: lnum,
                    line: line.trim_end().to_string(),
                });

                if limit > 0 && results.len() >= limit {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }),
        );

        if let Err(e) = search_result {
            log::debug!("Search error in {}: {}", path.display(), e);
        }
    }

    Ok(results)
}

/// Check if a file has a binary extension we should skip.
fn is_binary_extension(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "png"
                | "jpg"
                | "jpeg"
                | "gif"
                | "bmp"
                | "ico"
                | "webp"
                | "svg"
                | "mp3"
                | "wav"
                | "flac"
                | "ogg"
                | "aac"
                | "mp4"
                | "mkv"
                | "avi"
                | "mov"
                | "webm"
                | "zip"
                | "tar"
                | "gz"
                | "7z"
                | "rar"
                | "bz2"
                | "xz"
                | "exe"
                | "dll"
                | "so"
                | "dylib"
                | "pdf"
                | "docx"
                | "pptx"
                | "xlsx"
                | "woff"
                | "woff2"
                | "ttf"
                | "otf"
                | "eot"
                | "sqlite"
                | "db"
        ),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_empty_query_returns_empty() {
        let dir = std::env::temp_dir();
        let results = search_content(&dir, "", 0).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_finds_content_matches() {
        let dir = std::env::temp_dir().join(format!("felix_content_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let _ = fs::create_dir_all(&dir);
        fs::write(dir.join("test.txt"), "line one\nHello World\nline three\n").unwrap();
        fs::write(dir.join("other.txt"), "nothing here\n").unwrap();

        let results = search_content(&dir, "hello", 0).unwrap();
        let matching: Vec<_> = results
            .iter()
            .filter(|r| r.path.file_name().map_or(false, |f| f == "test.txt"))
            .collect();
        assert_eq!(matching.len(), 1);
        assert_eq!(matching[0].line_number, 2);
        assert!(matching[0].line.contains("Hello World"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_respects_limit() {
        let dir = std::env::temp_dir().join(format!("felix_climit_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let _ = fs::create_dir_all(&dir);
        fs::write(dir.join("test.txt"), "match one\nmatch two\nmatch three\n").unwrap();

        let results = search_content(&dir, "match", 2).unwrap();
        assert_eq!(results.len(), 2);

        let _ = fs::remove_dir_all(&dir);
    }
}
