pub mod manager;

#[cfg(feature = "markdown")]
pub mod markdown;

#[cfg(feature = "pdf")]
pub mod pdf;

#[cfg(feature = "docx")]
pub mod docx;

#[cfg(feature = "pptx")]
pub mod pptx;
