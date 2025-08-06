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

use preprocess::preprocess_image;
use inference::run_inference;
use apply_mask::apply_mask;

use clap::{Parser, ValueEnum};
use std::env;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Convert images between PNG, JPEG, and WebP formats. Optionally remove backgrounds from PNGs using a pure Rust ONNX model.",
    long_about = "A fast, cross-platform CLI and TUI tool for converting images between PNG, JPEG, and WebP formats.\n\
    - Use the CLI for quick conversions, or run without arguments for an interactive TUI.\n\
    - Optionally remove backgrounds from PNG outputs using a bundled ONNX model (no system dependencies).\n\
    - All logic is pure Rust and self-contained.\n\
    \nEXAMPLES:\n  image_converter -i input.jpg -o output -f png --remove-bg\n  image_converter --input file.webp --output result --format jpg\n  image_converter\n\nSupported formats: jpg, png, webp.\nBackground removal only applies to PNG & WebP outputs.")]
struct Cli {
    #[arg(short, long, value_name = "FILE", help = "Input image file path (required)")]
    input: Option<String>,

    #[arg(short, long, value_name = "NAME", help = "Output file name, without extension (required)")]
    output: Option<String>,

    #[arg(short, long, value_enum, value_name = "FORMAT", help = "Output format: jpg, png, or webp (required)")]
    format: Option<Format>,

    #[arg(short = 'b', long, help = "Remove background (PNG output only)")]
    remove_bg: bool,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
enum Format {
    Jpg,
    Png,
    Webp,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (input_path, output_base, output_ext, remove_bg) = if args.len() > 1 {
        let cli = Cli::parse();
        if cli.input.is_some() && cli.output.is_some() && cli.format.is_some() {
            let ext = match cli.format.unwrap() {
                Format::Jpg => "jpg",
                Format::Png => "png",
                Format::Webp => "webp",
            };
            (cli.input.unwrap(), cli.output.unwrap(), ext, cli.remove_bg)
        } else {
            eprintln!("Missing required CLI arguments. Use --help for usage.");
            std::process::exit(1);
        }
    } else {
        let (input_path, output_base, selection) = ui::get_user_input();
        let formats = ["jpg", "png", "webp"];
        (input_path, output_base, formats[selection], false)
    };

    let input_ext = input_path.split('.').last().unwrap_or("").to_lowercase();
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

    if (output_ext == "png" || output_ext == "webp") && remove_bg {
        match preprocess_image(&input_path)
            .and_then(|input_tensor| run_inference(input_tensor))
            .and_then(|mask| apply_mask(&input_path, mask, &output_file)) {
            Ok(_) => println!("Background removed and saved as {}", output_file),
            Err(e) => eprintln!("Background removal failed: {}", e),
        }
    }
}