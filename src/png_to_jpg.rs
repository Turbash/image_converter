use colored::*;
use image::{ImageFormat, DynamicImage, RgbImage};

pub fn png_to_jpg(input_path: &str, output_path: &str) -> Result<(), String> {
    println!("{} {} {}", "[INFO]".bold().yellow(), "ℹ".bold().blue(), format!("Converting PNG to JPG: {} -> {}", input_path, output_path));
    let img = match image::open(input_path) {
        Ok(i) => i,
        Err(e) => {
            return Err(format!("[ERROR] Failed to open input PNG '{}': {}", input_path, e));
        }
    };
    let rgb_img = if let Some(rgba) = img.as_rgba8() {
        let (w, h) = rgba.dimensions();
        let mut out = RgbImage::new(w, h);
        for (x, y, pixel) in rgba.enumerate_pixels() {
            let image::Rgba([r, g, b, a]) = *pixel;
            let alpha = a as f32 / 255.0;
            let white = 255.0;
            let r = (alpha * r as f32 + (1.0 - alpha) * white).round() as u8;
            let g = (alpha * g as f32 + (1.0 - alpha) * white).round() as u8;
            let b = (alpha * b as f32 + (1.0 - alpha) * white).round() as u8;
            out.put_pixel(x, y, image::Rgb([r, g, b]));
        }
        DynamicImage::ImageRgb8(out)
    } else {
        img.to_rgb8().into()
    };
    match rgb_img.save_with_format(output_path, ImageFormat::Jpeg) {
        Ok(_) => {
            println!("{} {} {}", "[SUCCESS]".bold().green(), "✔".green(), format!("Saved JPG: {}", output_path));
            Ok(())
        },
        Err(e) => Err(format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to save as JPG '{}': {}", output_path, e))),
    }
}