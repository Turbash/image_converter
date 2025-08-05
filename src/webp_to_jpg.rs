use image::{ImageFormat, DynamicImage, GenericImage};
use image::open;

pub fn webp_to_jpg(input_path: &str, output_path: &str) -> Result<(), String> {
    let img = match open(input_path) {
        Ok(i) => i,
        Err(e) => {
            return Err(format!("Failed to open WebP: {}", e));
        }
    };
    let img = if let Some(_) = img.as_rgba8() {
        let mut background = DynamicImage::new_rgb8(img.width(), img.height());
        match background.copy_from(&img, 0, 0) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("Failed to flatten WebP: {}", e));
            }
        }
        background
    } else {
        img
    };
    match img.save_with_format(output_path, ImageFormat::Jpeg) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to save as JPG: {}", e)),
    }
}
