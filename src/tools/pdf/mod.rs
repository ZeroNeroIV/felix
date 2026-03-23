//! PDF viewer module

pub fn can_handle(extension: &str) -> bool {
    extension == "pdf"
}

pub fn page_count(_path: &std::path::Path) -> Result<usize, Box<dyn std::error::Error>> {
    // TODO: Use a PDF crate for parsing
    todo!()
}
