use colored::*;
use image::{ImageFormat};
use image::open;

pub fn jpg_to_png(input_path: &str, output_path: &str) -> Result<(), String> {
    println!("{} {} {}", "[INFO]".bold().yellow(), "ℹ".bold().blue(), format!("Converting JPG to PNG: {} -> {}", input_path, output_path));
    let img = match open(input_path) {
        Ok(i) => i,
        Err(e) => {
            return Err(format!("[ERROR] Failed to open input JPG '{}': {}", input_path, e));
        }
    };
    match img.save_with_format(output_path, ImageFormat::Png) {
        Ok(_) => {
            println!("{} {} {}", "[SUCCESS]".bold().green(), "✔".green(), format!("Saved PNG: {}", output_path));
            Ok(())
        },
        Err(e) => Err(format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to save as PNG '{}': {}", output_path, e))),
    }
}