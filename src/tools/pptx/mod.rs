//! PPTX viewer/editor module

pub fn can_handle(extension: &str) -> bool {
    extension == "pptx"
}

pub fn slide_count(_path: &std::path::Path) -> Result<usize, Box<dyn std::error::Error>> {
    // TODO: Parse PPTX files
    todo!()
}
