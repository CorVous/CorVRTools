//! Camera frame sources.
//!
//! Real cameras are behind the `camera` feature. Without it, only the
//! synthetic [`TestPattern`] source is available, which keeps the rest of the
//! app buildable and testable on any machine.

use corvr_core::Frame;

#[cfg(feature = "camera")]
mod camera;
#[cfg(feature = "camera")]
pub use camera::{list_cameras, CameraInfo, CameraSource};

/// Errors from a frame source.
#[derive(Debug, thiserror::Error)]
pub enum CaptureError {
    #[error("camera not found: {0}")]
    NotFound(String),
    #[error("camera error: {0}")]
    Backend(String),
}

/// Anything that produces grayscale frames.
///
/// Not `Send`: some camera backends must stay on the thread that opened
/// them, so each source is opened on the thread that reads it.
pub trait FrameSource {
    /// Blocks until the next frame is ready.
    fn next_frame(&mut self) -> Result<Frame, CaptureError>;
}

/// A moving gradient, for running the app without hardware.
pub struct TestPattern {
    width: u32,
    height: u32,
    frame: u64,
    fps: u32,
}

impl TestPattern {
    pub fn new(width: u32, height: u32, fps: u32) -> Self {
        Self {
            width,
            height,
            frame: 0,
            fps: fps.max(1),
        }
    }
}

impl FrameSource for TestPattern {
    fn next_frame(&mut self) -> Result<Frame, CaptureError> {
        let shift = self.frame as u32;
        let pixels = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| ((x + y + shift) % 256) as u8))
            .collect();
        let timestamp_us = self.frame * 1_000_000 / self.fps as u64;
        self.frame += 1;
        Ok(Frame::new(self.width, self.height, pixels, timestamp_us).expect("sizes match"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_advances() {
        let mut src = TestPattern::new(4, 3, 60);
        let a = src.next_frame().unwrap();
        let b = src.next_frame().unwrap();
        assert_eq!(a.pixels.len(), 12);
        assert_ne!(a.pixels, b.pixels);
        assert!(b.timestamp_us > a.timestamp_us);
    }
}
