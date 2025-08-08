use onnxruntime::{environment::Environment, GraphOptimizationLevel, tensor::OrtOwnedTensor};
use ndarray::Array4;
use std::path::Path;
use colored::*;

pub fn run_inference(input: Array4<f32>) -> Result<ndarray::Array2<f32>, Box<dyn std::error::Error>> {
    println!("{} {} {}", "[INFO]".bold().yellow(), "ℹ".bold().blue(), "Loading ONNX model from models/u2net.onnx");
    let environment = Environment::builder().with_name("u2net").build()
        .map_err(|e| format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to create ONNX environment: {}", e)))?;
    let model_path = Path::new("models/u2net.onnx");
    if !model_path.exists() {
        return Err(format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Model file not found at {:?}. Please ensure models/u2net.onnx exists.", model_path)).into());
    }
    let mut session = environment
        .new_session_builder()
        .map_err(|e| format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to create ONNX session builder: {}", e)))?
        .with_optimization_level(GraphOptimizationLevel::Basic)
        .map_err(|e| format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to set ONNX optimization level: {}", e)))?
        .with_model_from_file(&model_path)
        .map_err(|e| format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to load ONNX model: {}", e)))?;

    let outputs: Vec<OrtOwnedTensor<f32, _>> = session.run(vec![input])
        .map_err(|e| format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("ONNX inference failed: {}", e)))?;
    let mask = outputs[0].view().to_owned()
        .into_dimensionality::<ndarray::Ix4>()
        .map_err(|e| format!("{} {} {}", "[ERROR]".bold().red(), "✖".red(), format!("Failed to convert ONNX output: {}", e)))?
        .index_axis_move(ndarray::Axis(0), 0)
        .index_axis_move(ndarray::Axis(0), 0);

    println!("{} {} {}", "[SUCCESS]".bold().green(), "✔".green(), "ONNX inference completed.");
    Ok(mask)
}