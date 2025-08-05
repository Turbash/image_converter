use webp::Encoder;
use std::fs;
use image::open;

pub fn jpg_to_webp(input_path: &str, output_path: &str) -> Result<(), String> {
    let img = match open(input_path) {
        Ok(i) => i,
        Err(e) => {
            return Err(format!("Failed to open JPG: {}", e));
        }
    };
    let rgb = img.to_rgb8();
    let encoder = Encoder::from_rgb(&rgb, rgb.width(), rgb.height());
    let webp_data = encoder.encode_lossless();
    match fs::write(output_path, &*webp_data) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to save as WebP: {}", e)),
    }
}