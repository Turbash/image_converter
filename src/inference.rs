use onnxruntime::{environment::Environment, GraphOptimizationLevel, tensor::OrtOwnedTensor};
use ndarray::Array4;

pub fn run_inference(input: Array4<f32>) -> Result<ndarray::Array2<f32>, Box<dyn std::error::Error>> {
    let environment = Environment::builder().with_name("u2net").build()?;
    let mut session = environment
        .new_session_builder()?
        .with_optimization_level(GraphOptimizationLevel::Basic)?
        .with_model_from_file("models/u2net.onnx")?;

    let outputs: Vec<OrtOwnedTensor<f32, _>> = session.run(vec![input])?;
    let mask = outputs[0].view().to_owned()
        .into_dimensionality::<ndarray::Ix4>()?
        .index_axis_move(ndarray::Axis(0), 0)
        .index_axis_move(ndarray::Axis(0), 0);

    Ok(mask)
}