use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use gpui_kit::*;
use rapidraw_core::{DecodedBuffer, ImageAdjustments, generate_test_pattern};
use rapidraw_pipeline::RenderPipeline;

pub struct AppState {
    pub current_file: Option<PathBuf>,
    pub base_buffer: DecodedBuffer,
    pub adjustments: ImageAdjustments,
    pub render_image: Arc<RenderImage>,
    pub pipeline: RenderPipeline,
}

impl AppState {
    pub fn new() -> Self {
        let pipeline = RenderPipeline::new();
        let adjustments = ImageAdjustments::default();
        let base_buffer = generate_test_pattern(1920, 1080);
        let processed = pipeline.process(&base_buffer, &adjustments);

        let render_image = Self::buffer_to_render_image(&processed);

        Self {
            current_file: None,
            base_buffer,
            adjustments,
            render_image,
            pipeline,
        }
    }

    pub fn load_file(&mut self, path: PathBuf) -> Result<(), String> {
        let start = Instant::now();
        let loaded = rapidraw_core::load_image_file(&path)?;
        println!("[PERF] Image Load & Decode: {:.2?}", start.elapsed());

        self.base_buffer = loaded;
        self.current_file = Some(path);
        self.reprocess();
        Ok(())
    }

    pub fn set_exposure(&mut self, exposure: f32) {
        if (self.adjustments.exposure - exposure).abs() > 0.001 {
            self.adjustments.exposure = exposure;
            self.reprocess();
        }
    }

    pub fn set_contrast(&mut self, contrast: f32) {
        if (self.adjustments.contrast - contrast).abs() > 0.001 {
            self.adjustments.contrast = contrast;
            self.reprocess();
        }
    }

    pub fn reset_adjustments(&mut self) {
        self.adjustments = ImageAdjustments::default();
        self.reprocess();
    }

    fn reprocess(&mut self) {
        let total_start = Instant::now();
        let processed = self.pipeline.process(&self.base_buffer, &self.adjustments);
        self.render_image = Self::buffer_to_render_image(&processed);
        println!("[PERF] Total UI Update Loop: {:.2?}", total_start.elapsed());
    }

    fn buffer_to_render_image(buffer: &DecodedBuffer) -> Arc<RenderImage> {
        let start = Instant::now();
        let rgba_img =
            image::RgbaImage::from_raw(buffer.width, buffer.height, buffer.rgba_data.to_vec())
                .unwrap_or_else(|| image::RgbaImage::new(1, 1));

        let frame = image::Frame::new(rgba_img);
        let img = Arc::new(RenderImage::new(vec![frame]));
        println!("[PERF] GPU Texture Prep: {:.2?}", start.elapsed());
        img
    }
}
