use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAdjustments {
    pub exposure: f32,
    pub contrast: f32,
    pub temperature: f32,
    pub tint: f32,
}

impl Default for ImageAdjustments {
    fn default() -> Self {
        Self {
            exposure: 0.0,
            contrast: 1.0,
            temperature: 0.0,
            tint: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct DecodedBuffer {
    pub width: u32,
    pub height: u32,
    pub rgba_data: Arc<Vec<u8>>,
}

pub fn load_image_file<P: AsRef<Path>>(path: P) -> Result<DecodedBuffer, String> {
    let img = image::open(path).map_err(|e| format!("Failed to open image: {e}"))?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    Ok(DecodedBuffer {
        width,
        height,
        rgba_data: Arc::new(rgba.into_raw()),
    })
}

pub fn generate_test_pattern(width: u32, height: u32) -> DecodedBuffer {
    let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let r = ((x as f32 / width as f32) * 255.0) as u8;
            let g = ((y as f32 / height as f32) * 255.0) as u8;
            let b = 180u8;
            let a = 255u8;
            rgba_data.extend_from_slice(&[r, g, b, a]);
        }
    }

    DecodedBuffer {
        width,
        height,
        rgba_data: Arc::new(rgba_data),
    }
}
