//! File operations — copy, move, delete, rename

use std::path::Path;

#[allow(dead_code)]
pub fn copy(_src: &Path, _dst: &Path) -> Result<(), std::io::Error> {
    todo!()
}

#[allow(dead_code)]
pub fn move_file(_src: &Path, _dst: &Path) -> Result<(), std::io::Error> {
    todo!()
}

#[allow(dead_code)]
pub fn delete(_path: &Path) -> Result<(), std::io::Error> {
    todo!()
}

#[allow(dead_code)]
pub fn rename(_old: &Path, _new: &Path) -> Result<(), std::io::Error> {
    todo!()
}
