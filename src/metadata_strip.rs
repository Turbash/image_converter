use image::{DynamicImage, ImageFormat};

pub fn strip_metadata_basic(input_path: &str, output_path: &str, format: ImageFormat) -> Result<(), String> {
    let img = image::open(input_path).map_err(|e| format!("Failed to open image: {}", e))?;
    img.save_with_format(output_path, format).map_err(|e| format!("Failed to save image: {}", e))
}