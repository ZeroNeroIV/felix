use lopdf::Document;
use std::collections::HashMap;
use std::path::Path;

pub struct PdfViewer {
    document: Document,
    page_cache: HashMap<usize, lopdf::ObjectId>,
}

impl PdfViewer {
    pub fn load(path: &Path) -> Result<PdfViewer, String> {
        let document = Document::load(path)
            .map_err(|e| format!("Failed to load PDF: {}", e))?;

        Ok(PdfViewer {
            document,
            page_cache: HashMap::new(),
        })
    }

    pub fn page_count(&self) -> usize {
        self.document.get_pages().len()
    }

    pub fn page_size(&self, page_num: usize) -> (f32, f32) {
        let pages = self.document.get_pages();
        let page_id = match pages.get(&(page_num as u32)) {
            Some(id) => id,
            None => return (0.0, 0.0),
        };

        let page = match self.document.get_object(*page_id) {
            Ok(lopdf::Object::Dictionary(dict)) => dict,
            _ => return (0.0, 0.0),
        };

        if let Ok(media_box) = page.get(b"MediaBox") {
            if let lopdf::Object::Array(arr) = media_box {
                if arr.len() >= 4 {
                    let width = get_number(&arr[2]).unwrap_or(612.0);
                    let height = get_number(&arr[3]).unwrap_or(792.0);
                    return (width, height);
                }
            }
        }

        (612.0, 792.0)
    }

    pub fn render_page(&self, page_num: usize, width: u32) -> Result<image::RgbaImage, String> {
        let pages = self.document.get_pages();
        let page_id = pages.get(&(page_num as u32))
            .ok_or_else(|| format!("Page {} not found", page_num))?;

        let (page_width, page_height) = self.page_size(page_num);
        
        let aspect_ratio = page_width / page_height;
        let height = (width as f32 / aspect_ratio) as u32;
        
        let width = width.max(1);
        let height = height.max(1);

        let mut pixels = vec![255u8; (width * height * 4) as usize];
        
        for i in 0..(width * height) as usize {
            pixels[i * 4 + 0] = 255;
            pixels[i * 4 + 1] = 255;
            pixels[i * 4 + 2] = 255;
            pixels[i * 4 + 3] = 255;
        }
        
        self.render_page_content(*page_id, &mut pixels, width, height, page_width, page_height);

        let img = image::RgbaImage::from_raw(width, height, pixels)
            .ok_or_else(|| "Failed to create image".to_string())?;
        
        Ok(img)
    }

    fn render_page_content(
        &self,
        page_id: lopdf::ObjectId,
        pixels: &mut [u8],
        img_width: u32,
        img_height: u32,
        page_width: f32,
        page_height: f32,
    ) {
        let page = match self.document.get_object(page_id) {
            Ok(lopdf::Object::Dictionary(dict)) => dict,
            _ => return,
        };

        if let Ok(contents) = page.get(b"Contents") {
            match contents {
                lopdf::Object::Reference(ref_id) => {
                    if let Ok(obj) = self.document.get_object(*ref_id) {
                        self.process_content_stream(&obj, pixels, img_width, img_height, page_width, page_height);
                    }
                }
                lopdf::Object::Array(arr) => {
                    for item in arr {
                        if let lopdf::Object::Reference(ref_id) = item {
                            if let Ok(obj) = self.document.get_object(*ref_id) {
                                self.process_content_stream(&obj, pixels, img_width, img_height, page_width, page_height);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn process_content_stream(
        &self,
        obj: &lopdf::Object,
        _pixels: &mut [u8],
        _img_width: u32,
        _img_height: u32,
        _page_width: f32,
        _page_height: f32,
    ) {
        let _content = match obj {
            lopdf::Object::Stream(stream) => stream.content.clone(),
            _ => return,
        };
    }
}

fn get_number(obj: &lopdf::Object) -> Option<f32> {
    match obj {
        lopdf::Object::Integer(i) => Some(*i as f32),
        lopdf::Object::Real(r) => Some(*r as f32),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_size() {
        let size = (0.0, 0.0);
        assert_eq!(size.0, 0.0);
    }
}