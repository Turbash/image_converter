use image::{io::Reader as ImageReader};
use ndarray::Array4;

pub fn preprocess_image(path: &str) -> Result<Array4<f32>, Box<dyn std::error::Error>> {
    let img = ImageReader::open(path)?.decode()?.resize_exact(320, 320, image::imageops::FilterType::Triangle).to_rgb8();

    let mut array = Array4::<f32>::zeros((1, 3, 320, 320));

    for (x, y, pixel) in img.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        array[[0, 0, y as usize, x as usize]] = r as f32 / 255.0;
        array[[0, 1, y as usize, x as usize]] = g as f32 / 255.0;
        array[[0, 2, y as usize, x as usize]] = b as f32 / 255.0;
    }

    Ok(array)
}
