use image::{ImageFormat};
use image::open;

pub fn jpg_to_png(input_path: &str, output_path: &str) -> Result<(), String> {
    let img = match open(input_path) {
        Ok(i) => i,
        Err(e) => {
            return Err(format!("Failed to open JPG: {}", e));
        }
    };
    match img.save_with_format(output_path, ImageFormat::Png) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to save as PNG: {}", e)),
    }
}