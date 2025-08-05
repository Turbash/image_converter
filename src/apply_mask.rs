use image::io::Reader as ImageReader;
use image::{RgbaImage, Rgba};

pub fn apply_mask(original_path: &str, mask: ndarray::Array2<f32>, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let orig = ImageReader::open(original_path)?.decode()?.resize_exact(320, 320, image::imageops::FilterType::Triangle).to_rgb8();

    let mut output: RgbaImage = RgbaImage::new(320, 320);
    for (x, y, pixel) in orig.enumerate_pixels() {
        let alpha = (mask[[y as usize, x as usize]] * 255.0).clamp(0.0, 255.0) as u8;
        let [r, g, b] = pixel.0;
        output.put_pixel(x, y, Rgba([r, g, b, alpha]));
    }

    output.save(output_path)?;
    Ok(())
}