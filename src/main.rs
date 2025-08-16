use colored::*;
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
mod palette_extract;

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
    about = "Convert images between PNG, JPEG, and WebP formats. Optionally remove backgrounds, strip metadata, or extract color palettes.",
    long_about = "A fast, cross-platform CLI and TUI tool for converting images between PNG, JPEG, and WebP formats.\n\
    - Use the CLI for quick conversions, or run without arguments for an interactive TUI.\n\
    - Optionally remove backgrounds from PNG/WebP outputs using a bundled ONNX model (no system dependencies).\n\
    - Optionally strip all metadata from output images (pure Rust, no dependencies).\n\
    - Optionally extract a color palette from any image.\n\
    - All logic is pure Rust and self-contained.\n\
    \nEXAMPLES:\n  image_converter -i input.jpg -o output -f png --remove-bg\n  image_converter -i input.png -o output -f png --strip-metadata\n  image_converter --input file.webp --output result --format jpg -p\n  image_converter\n\nSupported formats: jpg, jpeg, png, webp.\nBackground removal only applies to PNG & WebP outputs. Metadata stripping applies to all formats.")]
struct Cli {
    #[arg(short, long, value_name = "FILE", help = "Input image file path (required)")]
    input: Option<String>,

    #[arg(short, long, value_name = "PATH", help = "Output file path or directory (required)")]
    output: Option<String>,

    #[arg(short, long, value_enum, value_name = "FORMAT", help = "Output format: jpg, png, or webp (required)")]
    format: Option<Format>,

    #[arg(short = 'b', long, help = "Remove background (PNG/WebP output only)")]
    remove_bg: bool,

    #[arg(short = 's', long, help = "Strip all metadata from the output image (pure Rust, all formats)")]
    strip_metadata: bool,

    #[arg(short = 'p', long, help = "Extract and display a color palette from the input image")]
    palette: bool,
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
    let (input_path, output_base, output_ext, remove_bg, strip_metadata, _palette) = if args.len() > 1 {
        let cli = Cli::parse();
        if cli.input.is_some() && cli.output.is_some() && cli.format.is_some() {
            let ext = match cli.format.unwrap() {
                Format::Jpg => "jpg",
                Format::Png => "png",
                Format::Webp => "webp",
            };
            let input_path = cli.input.unwrap();
            let output_arg = cli.output.unwrap();
            let output_base = {
                use std::path::{Path, PathBuf};
                let output_path = Path::new(&output_arg);
                if output_path.is_dir() || (output_path.exists() && output_path.is_dir()) {
                    let input_file = Path::new(&input_path)
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "output".to_string());
                    let mut out = PathBuf::from(output_path);
                    out.push(input_file);
                    out.to_string_lossy().to_string()
                } else {
                    let stem = output_path.file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| output_arg.clone());
                    let parent = output_path.parent().unwrap_or_else(|| Path::new("."));
                    let mut out = parent.to_path_buf();
                    out.push(stem);
                    out.to_string_lossy().to_string()
                }
            };
            (input_path, output_base, ext, cli.remove_bg, cli.strip_metadata, cli.palette)
        } else {
            eprintln!("[ERROR] Missing required CLI arguments. Use --help for usage.");
            std::process::exit(1);
        }
    } else {
        let (input_path, output_base, selection, remove_bg, strip_metadata) = ui::get_user_input();
        let formats = ["jpg", "png", "webp"];
        (input_path, output_base, formats[selection], remove_bg, strip_metadata, false)
    };

    let input_ext = input_path.split('.').last().unwrap_or("").to_lowercase();
    let input_ext = if input_ext == "jpeg" { "jpg".to_string() } else { input_ext };
    let output_ext = if output_ext == "jpeg" { "jpg" } else { output_ext };
    let output_file = format!("{}.{}", output_base, output_ext);
    let do_copy = input_ext == output_ext && !remove_bg && !strip_metadata;
    if do_copy {
        match fs::copy(&input_path, &output_file) {
            Ok(_) => println!("\n{} {} {}", "[INFO]".bold().yellow(), "ℹ".bold().blue(), format!("No conversion needed. File copied as {}", output_file)),
            Err(e) => {
                eprintln!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to copy file: {}", e));
                process::exit(1);
            }
        }
        return;
    }

    println!("{} {} {}", "[INFO]".bold().yellow(), "ℹ".bold().blue(), format!("Starting conversion: {} -> {} ({} -> {})", input_path, output_file, input_ext, output_ext));
    let input_is_jpg = input_ext == "jpg" || input_ext == "jpeg";
    let output_is_jpg = output_ext == "jpg" || output_ext == "jpeg";

    let result = if input_ext == output_ext {
        Ok(())
    } else {
        match ((input_ext.as_str(), output_ext), (input_is_jpg, output_is_jpg)) {
            (("png", out), (_, true)) if out == "jpg" || out == "jpeg" => png_to_jpg::png_to_jpg(&input_path, &output_file),
            ((inp, "png"), (true, _)) if inp == "jpg" || inp == "jpeg" => jpg_to_png::jpg_to_png(&input_path, &output_file),
            (("webp", out), (_, true)) if out == "jpg" || out == "jpeg" => webp_to_jpg::webp_to_jpg(&input_path, &output_file),
            ((inp, "webp"), (true, _)) if inp == "jpg" || inp == "jpeg" => jpg_to_webp::jpg_to_webp(&input_path, &output_file),
            (("png", "webp"), _) => png_to_webp::png_to_webp(&input_path, &output_file),
            (("webp", "png"), _) => webp_to_png::webp_to_png(&input_path, &output_file),
            _ => Err(format!("[ERROR] Conversion from {} to {} is not supported.", input_ext, output_ext)),
        }
    };
    match result {
        Ok(_) => println!("\n{} {} {}\n  Input: {}\n  Output: {}\n  Format: {}",
            "[SUCCESS]".bold().green(), "✔".green(), "Conversion successful!",
            input_path, output_file, output_ext.to_uppercase()),
        Err(e) => {
            eprintln!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), e);
            process::exit(1);
        }
    }

    if (output_ext == "png" || output_ext == "webp") && remove_bg {
        println!("{} {} {}", "[INFO]".bold().yellow(), "ℹ".bold().blue(), "Attempting background removal...");
        match preprocess_image(&input_path)
            .and_then(|input_tensor| run_inference(input_tensor))
            .and_then(|mask| apply_mask(&input_path, mask, &output_file)) {
            Ok(_) => println!("{} {} {}", "[SUCCESS]".bold().green(), "✔".green(), format!("Background removed and saved as {}", output_file)),
            Err(e) => eprintln!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Background removal failed: {}", e)),
        }
    }
}