//! File operations — copy, move, delete, rename

use std::path::Path;

pub fn copy(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
    todo!()
}

pub fn move_file(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
    todo!()
}

pub fn delete(path: &Path) -> Result<(), std::io::Error> {
    todo!()
}

pub fn rename(old: &Path, new: &Path) -> Result<(), std::io::Error> {
    todo!()
}
