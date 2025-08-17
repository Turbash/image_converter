
use colored::*;
use std::fs;
use std::path::{Path};
use crate::{preprocess::preprocess_image, inference::run_inference, apply_mask::apply_mask};
use crate::png_to_jpg;
use crate::jpg_to_png;
use crate::webp_to_jpg;
use crate::jpg_to_webp;
use crate::png_to_webp;
use crate::webp_to_png;

pub struct BatchJob {
    pub input_dir: String,
    pub output_dir: String,
    pub format_index: usize,
    pub remove_bg: bool,
    pub strip_metadata: bool,
}

impl BatchJob {
    pub fn run(&self) {
        let formats = ["jpg", "png", "webp"];
        let output_ext = formats[self.format_index];
        let input_dir = Path::new(&self.input_dir);
        let output_dir = Path::new(&self.output_dir);
        let mut count = 0;
        let mut errors = 0;
        if !output_dir.exists() {
            if let Err(e) = fs::create_dir_all(output_dir) {
                eprintln!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to create output dir: {}", e));
                return;
            }
        }
        let entries = match fs::read_dir(input_dir) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to read input dir: {}", e));
                return;
            }
        };
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() { continue; }
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            let ext = if ext == "jpeg" { "jpg".to_string() } else { ext };
            if !["jpg", "png", "webp"].contains(&ext.as_str()) { continue; }
            let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
            let out_path = output_dir.join(format!("{}.{}", file_stem, output_ext));
            let input_path = path.display().to_string();
            let output_path = out_path.display().to_string();
            println!("{} {} Processing: {} -> {}", "[BATCH]".bold().cyan(), "→".cyan(), input_path, output_path);
            let result = if ext == output_ext {
                Ok(()) 
            } else {
                match ((ext.as_str(), output_ext), (ext == "jpg", output_ext == "jpg")) {
                    (("png", out), (_, true)) if out == "jpg" => png_to_jpg::png_to_jpg(&input_path, &output_path),
                    ((inp, "png"), (true, _)) if inp == "jpg" => jpg_to_png::jpg_to_png(&input_path, &output_path),
                    (("webp", out), (_, true)) if out == "jpg" => webp_to_jpg::webp_to_jpg(&input_path, &output_path),
                    ((inp, "webp"), (true, _)) if inp == "jpg" => jpg_to_webp::jpg_to_webp(&input_path, &output_path),
                    (("png", "webp"), _) => png_to_webp::png_to_webp(&input_path, &output_path),
                    (("webp", "png"), _) => webp_to_png::webp_to_png(&input_path, &output_path),
                    _ => Err(format!("[ERROR] Conversion from {} to {} is not supported.", ext, output_ext)),
                }
            };
            match result {
                Ok(_) => {
                    if (output_ext == "png" || output_ext == "webp") && self.remove_bg {
                        match preprocess_image(&input_path)
                            .and_then(|input_tensor| run_inference(input_tensor))
                            .and_then(|mask| apply_mask(&input_path, mask, &output_path)) {
                            Ok(_) => println!("{} {} {}", "[BATCH]".bold().green(), "✔".green(), format!("Background removed: {}", output_path)),
                            Err(e) => {
                                eprintln!("{} {} {}", "[BATCH]".bold().red(), "✖".red(), format!("BG removal failed: {}", e));
                                errors += 1;
                                continue;
                            }
                        }
                    }
                    if self.strip_metadata {
                        println!("{} {} {}", "[BATCH]".bold().yellow(), "ℹ".yellow(), format!("[TODO] Strip metadata: {}", output_path));
                    }
                    count += 1;
                }
                Err(e) => {
                    eprintln!("{} {} {}", "[BATCH]".bold().red(), "✖".red(), e);
                    errors += 1;
                }
            }
        }
        println!("\n{} {} Batch complete. {} files processed, {} errors.", "[BATCH]".bold().green(), "✔".green(), count, errors);
    }
}
