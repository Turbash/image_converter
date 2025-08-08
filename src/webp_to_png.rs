use colored::*;
use image::{ImageFormat, RgbaImage, DynamicImage, Rgba};
use image::{imageops,open};

pub fn webp_to_png(input_path: &str, output_path: &str) -> Result<(), String> {
    println!("{} {} {}", "[INFO]".bold().yellow(), "ℹ".bold().blue(), format!("Converting WebP to PNG: {} -> {}", input_path, output_path));
    let img = match open(input_path) {
        Ok(i) => i,
        Err(e) => {
            return Err(format!("[ERROR] Failed to open input WebP '{}': {}", input_path, e));
        }
    };
    let mut transparent = RgbaImage::new(img.width(), img.height());
    for pixel in transparent.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]);
    }
    let rgba = img.to_rgba8();
    imageops::overlay(&mut transparent, &rgba, 0, 0);
    let dyn_img = DynamicImage::ImageRgba8(transparent);
    match dyn_img.save_with_format(output_path, ImageFormat::Png) {
        Ok(_) => {
            println!("{} {} {}", "[SUCCESS]".bold().green(), "✔".green(), format!("Saved PNG: {}", output_path));
            Ok(())
        },
        Err(e) => Err(format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to save as PNG '{}': {}", output_path, e))),
    }
}