//! Filename search — fast recursive filename matching using ignore crate

use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// Search for files whose names contain the query string (case-insensitive).
/// Uses the ignore crate for fast traversal respecting .gitignore rules.
/// Returns up to `limit` results (0 = unlimited).
#[allow(dead_code)]
pub fn search_filenames(
    dir: &Path,
    query: &str,
    limit: usize,
) -> Result<Vec<PathBuf>, std::io::Error> {
    if query.is_empty() {
        return Ok(Vec::new());
    }

    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    let walker = WalkBuilder::new(dir)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    for entry in walker.flatten() {
        let file_name = match entry.file_name().to_str() {
            Some(name) => name,
            None => continue,
        };

        if file_name.to_lowercase().contains(&query_lower) {
            results.push(entry.into_path());

            if limit > 0 && results.len() >= limit {
                break;
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_empty_query_returns_empty() {
        let dir = std::env::temp_dir();
        let results = search_filenames(&dir, "", 0).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_finds_matching_filenames() {
        let dir = std::env::temp_dir().join(format!("felix_fname_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let _ = fs::create_dir_all(&dir);
        fs::write(dir.join("hello_world.txt"), "test").unwrap();
        fs::write(dir.join("goodbye.md"), "test").unwrap();
        fs::write(dir.join("HELLO_UPPER.rs"), "test").unwrap();

        let results = search_filenames(&dir, "hello", 0).unwrap();
        assert_eq!(results.len(), 2);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_respects_limit() {
        let dir = std::env::temp_dir().join(format!("felix_limit_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let _ = fs::create_dir_all(&dir);
        fs::write(dir.join("test_a.txt"), "test").unwrap();
        fs::write(dir.join("test_b.txt"), "test").unwrap();
        fs::write(dir.join("test_c.txt"), "test").unwrap();

        let results = search_filenames(&dir, "test", 2).unwrap();
        assert_eq!(results.len(), 2);

        let _ = fs::remove_dir_all(&dir);
    }
}
