mod png_to_jpg;
mod jpg_to_png;
mod webp_to_jpg;
mod jpg_to_webp;
mod png_to_webp;
mod webp_to_png;
mod ui;
mod preprocess;
mod inference;
mod apply_mask;

use std::fs;
use std::process;
use ui::get_user_input;
use preprocess::preprocess_image;
use inference::run_inference;
use apply_mask::apply_mask;

fn main() {
    let (input_path, output_base, selection) = get_user_input();
    let formats = ["jpg", "png", "webp"];
    let input_ext = input_path.split('.').last().unwrap_or("").to_lowercase();
    let output_ext = formats[selection];
    let output_file = format!("{}.{}", output_base, output_ext);

    if input_ext == output_ext {
        fs::copy(&input_path, &output_file).expect("Failed to copy file");
        println!("\n✅ No conversion needed. File copied as {}", output_file);
        return;
    }

    let result = match (input_ext.as_str(), output_ext) {
        ("png", "jpg") => png_to_jpg::png_to_jpg(&input_path, &output_file),
        ("jpg", "png") => jpg_to_png::jpg_to_png(&input_path, &output_file),
        ("webp", "jpg") => webp_to_jpg::webp_to_jpg(&input_path, &output_file),
        ("jpg", "webp") => jpg_to_webp::jpg_to_webp(&input_path, &output_file),
        ("png", "webp") => png_to_webp::png_to_webp(&input_path, &output_file),
        ("webp", "png") => webp_to_png::webp_to_png(&input_path, &output_file),
        _ => Err(format!("Conversion from {} to {} is not supported yet.", input_ext, output_ext)),
    };

    match result {
        Ok(_) => println!("\n✅ Conversion successful!\nInput: {}\nOutput: {}\nFormat: {}", input_path, output_file, output_ext.to_uppercase()),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }

    let input_img = match image::open(&input_path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Error: Failed to open input image: {}", e);
            std::process::exit(1);
        }
    };
    if output_ext == "png" {
        let model_path = "./model_simplified.onnx";
        match preprocess_image(&input_path)
            .and_then(|input_tensor| run_inference(input_tensor))
            .and_then(|mask| apply_mask(&input_path, mask, &output_file)) {
            Ok(_) => println!("Background removed and saved as {}", output_file),
            Err(e) => eprintln!("Background removal failed: {}", e),
        }
    }
}