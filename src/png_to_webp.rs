use webp;
use std::fs;

pub fn png_to_webp(input_path: &str, output_path: &str) -> Result<(), String> {
    let img = match image::open(input_path) {
        Ok(i) => i,
        Err(e) => {
            return Err(format!("Failed to open PNG: {}", e));
        }
    };
    let rgba = img.to_rgba8();
    let encoder = webp::Encoder::from_rgba(&rgba, rgba.width(), rgba.height());
    let webp_data = encoder.encode_lossless();
    match fs::write(output_path, &*webp_data) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to save as WebP: {}", e)),
    }
    // Background removal logic (not used, failed miserably, image rewriting on transparent background will rewrite the background as well, of course it does not work):
    // let mut transparent = RgbaImage::new(img.width(), img.height());
    // for pixel in transparent.pixels_mut() {
    //     *pixel = Rgba([0, 0, 0, 0]);
    // }
    // let rgba = img.to_rgba8();
    // image::imageops::overlay(&mut transparent, &rgba, 0, 0);
    // let dyn_img = image::DynamicImage::ImageRgba8(transparent);
    // match dyn_img.save_with_format(output_path, ImageFormat::WebP) {
    //     Ok(_) => Ok(()),
    //     Err(e) => Err(format!("Failed to save as WebP: {}", e)),
    // }
}