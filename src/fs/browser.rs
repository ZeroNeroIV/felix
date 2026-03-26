//! File system browser — navigation, listing, metadata

use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<SystemTime>,
}

impl FileEntry {
    /// Create a FileEntry from a path, reading metadata.
    pub fn from_path(path: &Path) -> Option<Self> {
        let metadata = path.metadata().ok()?;
        let name = path.file_name()?.to_string_lossy().to_string();
        Some(FileEntry {
            name,
            path: path.to_path_buf(),
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            modified: metadata.modified().ok(),
        })
    }

    pub fn icon(&self) -> &str {
        if self.is_dir {
            return "📁";
        }
        match self.path.extension().and_then(|e| e.to_str()) {
            Some("md") | Some("markdown") => "📝",
            Some("pdf") => "📄",
            Some("docx") => "📃",
            Some("pptx") => "📊",
            Some("rs") => "🦀",
            Some("py") => "🐍",
            Some("js") | Some("ts") => "📜",
            Some("toml") | Some("yaml") | Some("json") => "⚙️",
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") => "🖼️",
            Some("zip") | Some("tar") | Some("gz") | Some("7z") => "📦",
            Some("mp3") | Some("wav") | Some("flac") => "🎵",
            Some("mp4") | Some("mkv") | Some("avi") => "🎬",
            Some("sh") | Some("bash") => "⌨️",
            _ => "📄",
        }
    }

    pub fn size_display(&self) -> String {
        if self.is_dir {
            return "--".to_string();
        }
        let size = self.size;
        if size < 1024 {
            format!("{} B", size)
        } else if size < 1024 * 1024 {
            format!("{:.1} KB", size as f64 / 1024.0)
        } else if size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }

    pub fn modified_display(&self) -> String {
        match self.modified {
            Some(time) => {
                match time.duration_since(SystemTime::UNIX_EPOCH) {
                    Ok(duration) => {
                        let secs = duration.as_secs();
                        // Simple date display: YYYY-MM-DD HH:MM
                        let days = secs / 86400;
                        let years = 1970 + days / 365;
                        let remaining_days = days % 365;
                        let months = remaining_days / 30;
                        let day = (remaining_days % 30) + 1;
                        let hours = (secs % 86400) / 3600;
                        let minutes = (secs % 3600) / 60;
                        format!(
                            "{:04}-{:02}-{:02} {:02}:{:02}",
                            years,
                            months + 1,
                            day,
                            hours,
                            minutes
                        )
                    }
                    Err(_) => "Unknown".to_string(),
                }
            }
            None => "--".to_string(),
        }
    }
}

/// List directory contents, returning sorted entries (dirs first, then files, alphabetical)
pub fn list_directory(path: &Path) -> Result<Vec<FileEntry>, std::io::Error> {
    let mut entries = Vec::new();

    if !path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Not a directory: {}", path.display()),
        ));
    }

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files starting with '.'
        if name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata()?;
        let modified = metadata.modified().ok();

        entries.push(FileEntry {
            name,
            path: entry.path(),
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            modified,
        });
    }

    // Sort: directories first, then alphabetical
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(entries)
}

/// Get the parent directory
pub fn parent_dir(path: &Path) -> Option<PathBuf> {
    path.parent().map(|p| p.to_path_buf())
}

/// Get home directory
pub fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
}

/// Sorting fields
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    Name,
    Size,
    Modified,
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Sort entries by the specified field and direction
pub fn sort_entries(entries: &mut Vec<FileEntry>, field: SortField, direction: SortDirection) {
    let cmp = move |a: &FileEntry, b: &FileEntry| {
        // Always keep directories first
        if a.is_dir != b.is_dir {
            return if a.is_dir {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            };
        }

        let ord = match field {
            SortField::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            SortField::Size => a.size.cmp(&b.size),
            SortField::Modified => {
                match (a.modified, b.modified) {
                    (Some(at), Some(bt)) => at.cmp(&bt),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                }
            }
        };

        if direction == SortDirection::Descending {
            ord.reverse()
        } else {
            ord
        }
    };

    entries.sort_by(cmp);
}
