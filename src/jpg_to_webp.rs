use colored::*;
use webp::Encoder;
use std::fs;
use image::open;

pub fn jpg_to_webp(input_path: &str, output_path: &str) -> Result<(), String> {
    println!("{} {} {}", "[INFO]".bold().yellow(), "ℹ".bold().blue(), format!("Converting JPG to WebP: {} -> {}", input_path, output_path));
    let img = match open(input_path) {
        Ok(i) => i,
        Err(e) => {
            return Err(format!("[ERROR] Failed to open input JPG '{}': {}", input_path, e));
        }
    };
    let rgb = img.to_rgb8();
    let encoder = Encoder::from_rgb(&rgb, rgb.width(), rgb.height());
    let webp_data = encoder.encode_lossless();
    match fs::write(output_path, &*webp_data) {
        Ok(_) => {
            println!("{} {} {}", "[SUCCESS]".bold().green(), "✔".green(), format!("Saved WebP: {}", output_path));
            Ok(())
        },
        Err(e) => Err(format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to save as WebP '{}': {}", output_path, e))),
    }
}