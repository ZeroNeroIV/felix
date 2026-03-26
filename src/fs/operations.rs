//! File operations — copy, move, delete, rename

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Copy a file or directory recursively
pub fn copy(src: &Path, dst: &Path) -> Result<(), io::Error> {
    if src.is_dir() {
        copy_dir(src, dst)
    } else {
        fs::copy(src, dst).map(|_| ())
    }
}

/// Copy a directory recursively
fn copy_dir(src: &Path, dst: &Path) -> Result<(), io::Error> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

/// Move (rename) a file or directory
pub fn move_file(src: &Path, dst: &Path) -> Result<(), io::Error> {
    fs::rename(src, dst).or_else(|_| {
        // If rename fails across filesystems, fall back to copy + delete
        copy(src, dst)?;
        delete(src)
    })
}

/// Delete a file or directory recursively
pub fn delete(path: &Path) -> Result<(), io::Error> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

/// Rename a file or directory (same as move but with different semantics)
pub fn rename(old: &Path, new: &Path) -> Result<(), io::Error> {
    move_file(old, new)
}

/// Check if a path exists
pub fn exists(path: &Path) -> bool {
    path.exists()
}

/// Get the destination path for a copy/move operation
/// If dst is a directory, use the filename from src
pub fn resolve_dst(src: &Path, dst: &Path) -> PathBuf {
    if dst.is_dir() {
        dst.join(src.file_name().unwrap_or_default())
    } else {
        dst.to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_copy_file() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("source.txt");
        let dst = dir.path().join("dest.txt");
        
        fs::write(&src, "hello").unwrap();
        copy(&src, &dst).unwrap();
        
        assert!(dst.exists());
        assert_eq!(fs::read_to_string(&dst).unwrap(), "hello");
    }

    #[test]
    fn test_copy_dir() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("source_dir");
        let dst = dir.path().join("dest_dir");
        
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("file.txt"), "content").unwrap();
        
        copy(&src, &dst).unwrap();
        
        assert!(dst.is_dir());
        assert!(dst.join("file.txt").exists());
    }

    #[test]
    fn test_delete() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("to_delete.txt");
        
        fs::write(&file, "test").unwrap();
        delete(&file).unwrap();
        
        assert!(!file.exists());
    }

    #[test]
    fn test_rename() {
        let dir = tempdir().unwrap();
        let old = dir.path().join("old.txt");
        let new = dir.path().join("new.txt");
        
        fs::write(&old, "data").unwrap();
        rename(&old, &new).unwrap();
        
        assert!(!old.exists());
        assert!(new.exists());
        assert_eq!(fs::read_to_string(&new).unwrap(), "data");
    }
}
