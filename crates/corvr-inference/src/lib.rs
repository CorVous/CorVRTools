//! Trackers turn camera frames into [`FaceState`] updates.
//!
//! Each tracker owns one part of the face (mouth, eyes, or brows) and only
//! writes the fields for that part. Real model-backed trackers arrive in
//! later steps; the ONNX loader is behind the `onnx` feature.

use corvr_core::{Expression, FaceState, Frame};

#[cfg(feature = "onnx")]
pub mod onnx;

/// Errors from a tracker.
#[derive(Debug, thiserror::Error)]
pub enum TrackError {
    #[error("model error: {0}")]
    Model(String),
    #[error("unexpected input: {0}")]
    Input(String),
}

/// Something that reads a frame and updates part of the face.
pub trait Tracker: Send {
    /// Short name for logs, e.g. "mouth".
    fn name(&self) -> &str;

    /// Processes one frame and writes this tracker's fields into `state`.
    fn process(&mut self, frame: &Frame, state: &mut FaceState) -> Result<(), TrackError>;
}

/// Placeholder mouth tracker: opens and closes the jaw on a timer, so the
/// OSC path can be checked in VRChat before any model is wired in.
#[derive(Default)]
pub struct DemoMouthTracker;

impl Tracker for DemoMouthTracker {
    fn name(&self) -> &str {
        "demo-mouth"
    }

    fn process(&mut self, frame: &Frame, state: &mut FaceState) -> Result<(), TrackError> {
        let t = frame.timestamp_us as f32 / 1_000_000.0;
        let open = (t * std::f32::consts::PI).sin() * 0.5 + 0.5;
        state.set(Expression::JawOpen, open);
        state.lip_tracking_active = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_mouth_moves_jaw_only() {
        let mut t = DemoMouthTracker;
        let mut s = FaceState::default();
        let f = Frame::new(1, 1, vec![0], 500_000).unwrap();
        t.process(&f, &mut s).unwrap();
        assert!((s.get(Expression::JawOpen) - 1.0).abs() < 1e-4);
        assert!(s.lip_tracking_active);
        assert!(!s.eye_tracking_active);
        assert_eq!(s.get(Expression::BrowInnerUpLeft), 0.0);
    }
}
