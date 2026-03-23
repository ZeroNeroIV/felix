//! Markdown viewer/editor module

pub fn can_handle(extension: &str) -> bool {
    matches!(extension, "md" | "markdown" | "mdown" | "mkd")
}

pub fn render(content: &str) -> String {
    // TODO: Use pulldown-cmark for rendering
    content.to_string()
}
