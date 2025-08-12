use dialoguer::{Input, Select, theme::ColorfulTheme, Confirm};
use dialoguer::console::Style;
use std::path::{Path, PathBuf};
use std::fs;
use image::io::Reader as ImageReader;
use colored::*;
use crate::palette_extract;

pub fn get_user_input() -> (String, String, usize, bool, bool) {
    let cyan = Style::new().cyan().bold();
    println!("{}", cyan.apply_to("\n=== Image Converter TUI ===\n"));

    fn pick_file(start_dir: &Path) -> Option<PathBuf> {
        #[derive(Debug)]
        enum Item {
            Up,
            Dir(PathBuf),
            File(PathBuf),
        }
        let mut current_dir = start_dir.to_path_buf();
        loop {
            let mut entries: Vec<_> = fs::read_dir(&current_dir)
                .ok()?
                .filter_map(|e| e.ok())
                .filter(|e| {
                    if let Some(name) = e.file_name().to_str() {
                        !name.starts_with('.')
                    } else {
                        false
                    }
                })
                .collect();
            entries.sort_by_key(|e| e.path());
            let mut items = vec![];
            let mut actions = vec![];
            if let Some(_parent) = current_dir.parent() {
                items.push("[..]".to_string());
                actions.push(Item::Up);
            }
            for entry in &entries {
                let path = entry.path();
                if path.is_dir() {
                    items.push(format!("📁 {}", path.file_name()?.to_string_lossy()));
                    actions.push(Item::Dir(path));
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ["jpg", "jpeg", "png", "webp"].contains(&ext.to_lowercase().as_str()) {
                        items.push(path.file_name()?.to_string_lossy().to_string());
                        actions.push(Item::File(path));
                    }
                }
            }
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Select an image file (current: {})", current_dir.display()))
                .items(&items)
                .default(0)
                .interact()
                .ok()?;
            match &actions[selection] {
                Item::Up => {
                    current_dir = current_dir.parent().unwrap().to_path_buf();
                }
                Item::Dir(dir) => {
                    current_dir = dir.clone();
                }
                Item::File(file) => {
                    return Some(file.clone());
                }
            }
        }
    }

    let input_path = pick_file(&std::env::current_dir().unwrap())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| {
            eprintln!("[ERROR] No file selected. Exiting.");
            std::process::exit(1);
        });

    let show_palette = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Show color palette for this image?")
        .default(false)
        .interact()
        .unwrap_or(false);
    if show_palette {
        println!("\n{}", Style::new().cyan().apply_to("Extracting color palette..."));
        let img = match ImageReader::open(&input_path) {
            Ok(reader) => match reader.decode() {
                Ok(img) => img,
                Err(e) => {
                    eprintln!("[ERROR] Failed to decode image for palette extraction: {}", e);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("[ERROR] Failed to open image for palette extraction: {}", e);
                std::process::exit(1);
            }
        };
        let palette = palette_extract::extract_palette(&img, 6);
        println!("\n{}", Style::new().magenta().bold().apply_to("Dominant Color Palette:"));
        for hex in &palette {
            print!("{}  ", "  ".on_truecolor(
                u8::from_str_radix(&hex[1..3], 16).unwrap_or(0),
                u8::from_str_radix(&hex[3..5], 16).unwrap_or(0),
                u8::from_str_radix(&hex[5..7], 16).unwrap_or(0)
            ));
            print!("{}  ", hex.bold());
        }
        println!("\n");
    }

    let output_base = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the desired output file name (without extension)")
        .validate_with(|input: &String| {
            if input.trim().is_empty() {
                Err("Output name cannot be empty")
            } else if input.contains('.') {
                Err("Do not include an extension (e.g., .png)")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .unwrap_or_else(|e| {
            eprintln!("[ERROR] Failed to read output path: {}", e);
            std::process::exit(1);
        });

    let formats = ["JPG/JPEG", "PNG", "WebP"];
    let format_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the output format")
        .items(&formats)
        .default(0)
        .interact()
        .unwrap_or_else(|e| {
            eprintln!("[ERROR] Failed to select format: {}", e);
            std::process::exit(1);
        });

    let mut remove_bg = false;
    if format_index == 1 || format_index == 2 {
        println!("\n{}", Style::new().yellow().apply_to("Background removal is available for PNG and WebP outputs."));
        remove_bg = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Remove background from output image?")
            .default(false)
            .interact()
            .unwrap_or(false);
        if remove_bg {
            println!("{}", Style::new().green().apply_to("Background removal will be applied."));
        }
    }

    println!("\n{}", cyan.apply_to("Summary:"));
    println!("  Input file:   {}", input_path);
    println!("  Output name:  {}", output_base);
    println!("  Output type:  {}", formats[format_index]);
    if format_index == 1 || format_index == 2 {
        println!("  Remove BG:    {}", if remove_bg { "Yes" } else { "No" });
    }

    let proceed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with these settings?")
        .default(true)
        .interact()
        .unwrap_or_else(|e| {
            eprintln!("[ERROR] Failed to confirm operation: {}", e);
            std::process::exit(1);
        });
    if !proceed {
        println!("{}", Style::new().red().apply_to("Operation cancelled by user."));
        std::process::exit(0);
    }

    let show_palette = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Show color palette for this image?")
        .default(false)
        .interact()
        .unwrap_or(false);

    (input_path, output_base, format_index, remove_bg, show_palette)
}