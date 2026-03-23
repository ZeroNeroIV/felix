//! DOCX viewer/editor module

pub fn can_handle(extension: &str) -> bool {
    extension == "docx"
}

pub fn read(_path: &std::path::Path) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: Parse DOCX files
    todo!()
}
