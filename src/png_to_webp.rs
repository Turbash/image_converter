use webp;
use std::fs;

pub fn png_to_webp(input_path: &str, output_path: &str) -> Result<(), String> {
    println!("[INFO] Converting PNG to WebP: {} -> {}", input_path, output_path);
    let img = match image::open(input_path) {
        Ok(i) => i,
        Err(e) => {
            return Err(format!("[ERROR] Failed to open input PNG '{}': {}", input_path, e));
        }
    };
    let rgba = img.to_rgba8();
    let encoder = webp::Encoder::from_rgba(&rgba, rgba.width(), rgba.height());
    let webp_data = encoder.encode_lossless();
    match fs::write(output_path, &*webp_data) {
        Ok(_) => {
            println!("[SUCCESS] Saved WebP: {}", output_path);
            Ok(())
        },
        Err(e) => Err(format!("[ERROR] Failed to save as WebP '{}': {}", output_path, e)),
    }
}