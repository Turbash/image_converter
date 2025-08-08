
use image::io::Reader as ImageReader;
use image::{RgbaImage, Rgba, DynamicImage, imageops};
use ndarray::Array2;

pub fn apply_mask(original_path: &str, mask: Array2<f32>, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let orig_img = ImageReader::open(original_path)?.decode()?.to_rgb8();
    let (width, height) = orig_img.dimensions();

    let mask_img = {
        let mut mask_buf = image::GrayImage::new(mask.shape()[1] as u32, mask.shape()[0] as u32);
        for ((y, x), v) in mask.indexed_iter() {
            let alpha = (v * 255.0).clamp(0.0, 255.0) as u8;
            mask_buf.put_pixel(x as u32, y as u32, image::Luma([alpha]));
        }
        DynamicImage::ImageLuma8(mask_buf)
    };

    let resized_mask = mask_img.resize_exact(width, height, imageops::FilterType::Triangle).to_luma8();

    let mut output: RgbaImage = RgbaImage::new(width, height);
    for (x, y, pixel) in orig_img.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        let alpha = resized_mask.get_pixel(x, y)[0];
        output.put_pixel(x, y, Rgba([r, g, b, alpha]));
    }

    output.save(output_path)?;
    Ok(())
}