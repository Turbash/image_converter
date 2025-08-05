use dialoguer::{Input, Select};

pub fn get_user_input() -> (String, String, usize) {
    let input_path = Input::new()
        .with_prompt("Enter the path to your input image file")
        .interact_text()
        .expect("Failed to read input path");

    let output_base = Input::new()
        .with_prompt("Enter the desired output file path (without extension)")
        .interact_text()
        .expect("Failed to read output path");

    let formats = ["JPG", "PNG", "WebP"];
    let format_index = Select::new()
        .with_prompt("Select the output format")
        .items(&formats)
        .default(0)
        .interact()
        .expect("Failed to select format");

    (input_path, output_base, format_index)
}