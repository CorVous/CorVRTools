//! ONNX model loading through ONNX Runtime.
//!
//! Model files are not stored in this repo. They are loaded from a path the
//! user provides (see the README for where to get them).

use std::path::Path;

use corvr_core::Frame;
use ndarray::Array4;
use ort::environment::Environment;
use ort::session::Session;
use ort::value::TensorRef;

use crate::TrackError;

/// A loaded ONNX model that takes one grayscale image and returns a flat
/// list of floats.
pub struct OnnxModel {
    session: Session,
    _env: Environment,
    input_size: (u32, u32),
}

impl OnnxModel {
    /// Loads a model whose input is `[1, 1, height, width]` floats in 0..1.
    pub fn load(path: &Path, input_size: (u32, u32)) -> Result<Self, TrackError> {
        let model_err = |e: ort::Error| TrackError::Model(e.to_string());
        let env = ort::init()
            .with_name("corvrtools")
            .build()
            .map_err(model_err)?;
        let session = Session::builder(&env)
            .and_then(|mut b| b.commit_from_file(path))
            .map_err(model_err)?;
        Ok(Self {
            session,
            _env: env,
            input_size,
        })
    }

    /// Runs the model on a frame that is already resized to `input_size`.
    pub fn run(&mut self, frame: &Frame) -> Result<Vec<f32>, TrackError> {
        let (w, h) = self.input_size;
        if (frame.width, frame.height) != (w, h) {
            return Err(TrackError::Input(format!(
                "expected {w}x{h}, got {}x{}",
                frame.width, frame.height
            )));
        }
        let input = Array4::from_shape_fn((1, 1, h as usize, w as usize), |(_, _, y, x)| {
            frame.pixels[y * w as usize + x] as f32 / 255.0
        });
        let tensor =
            TensorRef::from_array_view(&input).map_err(|e| TrackError::Model(e.to_string()))?;
        let outputs = self
            .session
            .run(ort::inputs![tensor])
            .map_err(|e| TrackError::Model(e.to_string()))?;
        let (_, data) = outputs[0]
            .try_extract_tensor::<f32>()
            .map_err(|e| TrackError::Model(e.to_string()))?;
        Ok(data.to_vec())
    }
}
