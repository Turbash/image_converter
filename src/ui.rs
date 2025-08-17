pub fn pick_dir(start_dir: &std::path::Path) -> Option<std::path::PathBuf> {
    use dialoguer::{Select, theme::ColorfulTheme};
    use std::fs;
    use std::path::PathBuf;
    #[derive(Debug)]
    enum Item {
        SelectCurrent,
        Up,
        Dir(PathBuf),
    }
    let mut current_dir = start_dir.to_path_buf();
    loop {
        let mut entries: Vec<_> = fs::read_dir(&current_dir)
            .ok()?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir() && !e.file_name().to_str().map(|n| n.starts_with('.')).unwrap_or(false))
            .collect();
        entries.sort_by_key(|e| e.path());
        let mut items = vec!["✅ Select this directory".to_string()];
        let mut actions = vec![Item::SelectCurrent];
        if let Some(_parent) = current_dir.parent() {
            items.push("[..]".to_string());
            actions.push(Item::Up);
        }
        for entry in &entries {
            let path = entry.path();
            items.push(format!("📁 {}", path.file_name()?.to_string_lossy()));
            actions.push(Item::Dir(path));
        }
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Select directory (current: {})", current_dir.display()))
            .items(&items)
            .default(0)
            .interact()
            .ok()?;
        match &actions[selection] {
            Item::SelectCurrent => return Some(current_dir.clone()),
            Item::Up => {
                current_dir = current_dir.parent().unwrap().to_path_buf();
            }
            Item::Dir(dir) => {
                current_dir = dir.clone();
            }
        }
    }
}
pub struct BatchOptions {
    pub input_dir: String,
    pub output_dir: String,
    pub format_index: usize,
    pub remove_bg: bool,
    pub strip_metadata: bool,
}

pub fn get_batch_options() -> Option<BatchOptions> {
    let cyan = Style::new().cyan().bold();
    println!("{}", cyan.apply_to("\n=== Batch Processing ===\n"));

    let input_dir = {
        println!("Select input directory:");
        pick_dir(&std::env::current_dir().unwrap())
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| {
                eprintln!("[ERROR] No input directory selected. Exiting.");
                std::process::exit(1);
            })
    };
    let output_dir = {
        println!("Select output directory:");
        pick_dir(&std::env::current_dir().unwrap())
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| {
                eprintln!("[ERROR] No output directory selected. Exiting.");
                std::process::exit(1);
            })
    };
    let formats = ["JPG/JPEG", "PNG", "WebP"];
    let format_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the output format for all images")
        .items(&formats)
        .default(0)
        .interact()
        .unwrap_or(0);
    let mut remove_bg = false;
    if format_index == 1 || format_index == 2 {
        println!("\n{}", Style::new().yellow().apply_to("Background removal is available for PNG and WebP outputs."));
        remove_bg = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Remove background from output images?")
            .default(false)
            .interact()
            .unwrap_or(false);
    }
    let strip_metadata = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Strip all metadata from output images?")
        .default(false)
        .interact()
        .unwrap_or(false);
    println!("\n{}", cyan.apply_to("Batch Summary:"));
    println!("  Input dir:    {}", input_dir);
    println!("  Output dir:   {}", output_dir);
    println!("  Output type:  {}", formats[format_index]);
    if format_index == 1 || format_index == 2 {
        println!("  Remove BG:    {}", if remove_bg { "Yes" } else { "No" });
    }
    println!("  Strip metadata: {}", if strip_metadata { "Yes" } else { "No" });
    let proceed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with batch processing?")
        .default(true)
        .interact()
        .unwrap_or(false);
    if !proceed {
        println!("{}", Style::new().red().apply_to("Batch operation cancelled by user."));
        return None;
    }
    Some(BatchOptions {
        input_dir,
        output_dir,
        format_index,
        remove_bg,
        strip_metadata,
    })
}
pub enum TuiAction {
    SingleFile,
    Batch,
    Settings,
    Help,
    Exit,
}

pub fn main_menu() -> TuiAction {
    let cyan = Style::new().cyan().bold();
    println!("{}", cyan.apply_to("\n=== Image Converter ===\n"));
    let items = vec![
        "Single File Conversion",
        "Batch Processing",
        "Settings",
        "Help / About",
        "Exit",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option:")
        .items(&items)
        .default(0)
        .interact()
        .unwrap_or(4);
    match selection {
        0 => TuiAction::SingleFile,
        1 => TuiAction::Batch,
        2 => TuiAction::Settings,
        3 => TuiAction::Help,
        _ => TuiAction::Exit,
    }
}
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
                    items.push(format!("\u{1F4C1} {}", path.file_name()?.to_string_lossy()));
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

    let output_dir = pick_dir(&std::env::current_dir().unwrap())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| {
            eprintln!("[ERROR] No output directory selected. Exiting.");
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
    let output_base = format!("{}/{}", output_dir, output_base);

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

    let strip_metadata = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Strip all metadata from output image?")
        .default(false)
        .interact()
        .unwrap_or(false);

    println!("\n{}", cyan.apply_to("Summary:"));
    println!("  Input file:   {}", input_path);
    println!("  Output name:  {}", output_base);
    println!("  Output type:  {}", formats[format_index]);
    if format_index == 1 || format_index == 2 {
        println!("  Remove BG:    {}", if remove_bg { "Yes" } else { "No" });
    }
    println!("  Strip metadata: {}", if strip_metadata { "Yes" } else { "No" });

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
    (input_path, output_base, format_index, remove_bg, strip_metadata)
}

/// Show About/Help information in the TUI.
pub fn show_about_help() {
    use std::io::{self, Write};
    let cyan = Style::new().cyan().bold();
    let magenta = Style::new().magenta().bold();
    println!("{}", cyan.apply_to("\n=== About / Help ===\n"));
    println!("{}", magenta.apply_to("Image Converter - Rust CLI/TUI"));
    println!("Version: 1.0.0");
    println!("Author: Turbash Negi");
    println!("License: MIT\n");
    println!("Features:");
    println!("  - Convert between PNG, JPG/JPEG, and WebP formats");
    println!("  - ONNX-based background removal (PNG/WebP)");
    println!("  - Batch processing and single file mode");
    println!("  - Color palette extraction");
    println!("  - Metadata stripping");
    println!("  - TUI with file/directory explorer");
    println!("  - Robust error handling and colored logs\n");
    println!("Usage:");
    println!("  - Use arrow keys to navigate menus and select files/directories.");
    println!("  - Choose 'Single File Conversion' or 'Batch Processing' from the main menu.");
    println!("  - Follow prompts for options like background removal, metadata, and palette.");
    println!("  - Use the CLI for scripting: image_converter --help\n");
    println!("Credits:");
    println!("  - Built with Rust, image, webp, clap, dialoguer, onnxruntime, colored, kmeans-colors, palette crates.");
    println!("  - ONNX model: u2net.onnx (for background removal)");
    println!("  - Inspired by open-source image tools and the Rust community.\n");
    println!("For more info, see the README or run with --help.\n");
    print!("Press Enter to return to the main menu...");
    io::stdout().flush().ok();
    let _ = io::stdin().read_line(&mut String::new());

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


    let output_dir = pick_dir(&std::env::current_dir().unwrap())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| {
            eprintln!("[ERROR] No output directory selected. Exiting.");
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
    let output_base = format!("{}/{}", output_dir, output_base);

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

    let strip_metadata = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Strip all metadata from output image?")
        .default(false)
        .interact()
        .unwrap_or(false);

    println!("\n{}", cyan.apply_to("Summary:"));
    println!("  Input file:   {}", input_path);
    println!("  Output name:  {}", output_base);
    println!("  Output type:  {}", formats[format_index]);
    if format_index == 1 || format_index == 2 {
        println!("  Remove BG:    {}", if remove_bg { "Yes" } else { "No" });
    }
    println!("  Strip metadata: {}", if strip_metadata { "Yes" } else { "No" });

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
}