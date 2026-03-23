//! Markdown viewer/editor module

#[allow(dead_code)]
pub fn can_handle(extension: &str) -> bool {
    matches!(extension, "md" | "markdown" | "mdown" | "mkd")
}

#[allow(dead_code)]
pub fn render(content: &str) -> String {
    // TODO: Use pulldown-cmark for rendering
    content.to_string()
}
