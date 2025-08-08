
use image::{io::Reader as ImageReader};
use ndarray::Array4;
use colored::*;

pub fn preprocess_image(path: &str) -> Result<Array4<f32>, Box<dyn std::error::Error>> {
    println!("{} {} {}", "[INFO]".bold().yellow(), "[34m[1mℹ[0m".yellow(), format!("Preprocessing image for ONNX model: {}", path));
    let img = ImageReader::open(path)
        .map_err(|e| format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to open image '{}': {}", path, e)))?
        .decode()
        .map_err(|e| format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to decode image '{}': {}", path, e)))?
        .resize_exact(320, 320, image::imageops::FilterType::Triangle)
        .to_rgb8();

    let mut array = Array4::<f32>::zeros((1, 3, 320, 320));

    for (x, y, pixel) in img.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        array[[0, 0, y as usize, x as usize]] = r as f32 / 255.0;
        array[[0, 1, y as usize, x as usize]] = g as f32 / 255.0;
        array[[0, 2, y as usize, x as usize]] = b as f32 / 255.0;
    }

    println!("{} {} {}", "[SUCCESS]".bold().green(), "✔".green(), "Preprocessing complete.");
    Ok(array)
}