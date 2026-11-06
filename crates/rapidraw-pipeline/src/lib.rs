use rapidraw_core::{DecodedBuffer, ImageAdjustments};
use rayon::prelude::*;
use std::sync::Arc;
use std::time::Instant;

pub struct RenderPipeline;

impl RenderPipeline {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, input: &DecodedBuffer, adjustments: &ImageAdjustments) -> DecodedBuffer {
        let start = Instant::now();

        let mut lut = [0u8; 256];
        let exposure_factor = 2.0f32.powf(adjustments.exposure);
        let contrast_factor = adjustments.contrast;

        for i in 0..256 {
            let norm = i as f32 / 255.0;
            let exposed = norm * exposure_factor;
            let adjusted = (exposed - 0.5) * contrast_factor + 0.5;
            lut[i] = (adjusted.clamp(0.0, 1.0) * 255.0) as u8;
        }

        let mut output_data = vec![0u8; input.rgba_data.len()];

        output_data
            .par_chunks_exact_mut(4)
            .zip(input.rgba_data.par_chunks_exact(4))
            .for_each(|(out_pixel, in_pixel)| {
                out_pixel[0] = lut[in_pixel[0] as usize];
                out_pixel[1] = lut[in_pixel[1] as usize];
                out_pixel[2] = lut[in_pixel[2] as usize];
                out_pixel[3] = in_pixel[3];
            });

        println!(
            "[PERF] Pipeline Process: {:.2?} (Pixels: {}x{})",
            start.elapsed(),
            input.width,
            input.height
        );

        DecodedBuffer {
            width: input.width,
            height: input.height,
            rgba_data: Arc::new(output_data),
        }
    }
}
