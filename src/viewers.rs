use crate::config::ViewerConfig;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Image,
    Video,
    Pdf,
    Document,
    Directory,
    Other,
}

const IMAGE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "webp", "bmp", "svg", "tiff", "tif", "ico", "avif",
];

const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "m4v", "mpg", "mpeg",
];

pub fn detect_file_type(path: &Path) -> FileType {
    if path.is_dir() {
        return FileType::Directory;
    }

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    match extension.as_deref() {
        Some(ext) if IMAGE_EXTENSIONS.contains(&ext) => FileType::Image,
        Some(ext) if VIDEO_EXTENSIONS.contains(&ext) => FileType::Video,
        Some("pdf") => FileType::Pdf,
        Some(_) => FileType::Document,
        None => FileType::Other,
    }
}

pub fn get_viewer(file_type: FileType, config: &ViewerConfig) -> Option<&str> {
    match file_type {
        FileType::Image => Some(&config.image_viewer),
        FileType::Video => Some(&config.video_viewer),
        FileType::Pdf => Some(&config.pdf_viewer),
        _ => None,
    }
}

pub fn launch_viewer(file_type: FileType, path: &Path, config: &ViewerConfig) -> Result<(), String> {
    let viewer = get_viewer(file_type, config).ok_or_else(|| "No viewer available for this file type".to_string())?;

    let result = Command::new(viewer)
        .arg(path)
        .spawn();

    match result {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err(format!("Viewer not found: {}", viewer))
        }
        Err(e) => {
            Err(format!("Failed to launch {}: {}", viewer, e))
        }
    }
}