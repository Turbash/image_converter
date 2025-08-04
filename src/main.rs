use image::ImageFormat;

fn main() {
    let input_path = "input.png";
    let output_path = "output.jpg";

    let img = match image::open(input_path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Error: Failed to open input image: {}", e);
            std::process::exit(1);
        }
    };

    match img.save_with_format(output_path, ImageFormat::Jpeg) {
        Ok(_) => println!("Conversion successful: {} -> {}", input_path, output_path),
        Err(e) => {
            eprintln!("Error: Failed to save output image: {}", e);
            std::process::exit(1);
        }
    }
}