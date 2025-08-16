use image::DynamicImage;
use palette::{Srgb};
use kmeans_colors::get_kmeans;

pub fn extract_palette(img: &DynamicImage, num_colors: usize) -> Vec<String> {
    let small = if img.width() > 128 || img.height() > 128 {
        img.resize(128, 128, image::imageops::FilterType::Triangle)
    } else {
        img.clone()
    };
    let rgb_img = small.to_rgb8();
    let pixels: Vec<Srgb<u8>> = rgb_img.pixels().map(|p| Srgb::new(p[0], p[1], p[2])).collect();
    let data: Vec<Srgb<f32>> = pixels.iter().map(|&c| c.into_format::<f32>()).collect();
    let result = get_kmeans(
        20,
        num_colors,
        1e-3,
        false,
        &data,
        42,
    );
    let mut colors = vec![];
    for center in result.centroids {
        let color = center.into_format::<u8>();
        colors.push(format!("#{:02X}{:02X}{:02X}", color.red, color.green, color.blue));
    }
    colors
}