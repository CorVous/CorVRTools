use std::time::Instant;

use corvr_core::Frame;
use nokhwa::pixel_format::LumaFormat;
use nokhwa::utils::{ApiBackend, CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;

use crate::{CaptureError, FrameSource};

/// A camera the OS reports.
#[derive(Debug, Clone)]
pub struct CameraInfo {
    pub index: String,
    pub name: String,
    pub description: String,
}

/// Lists cameras the OS can see. Use this to find the Beyond 2e eye cameras
/// and the Babble camera.
pub fn list_cameras() -> Result<Vec<CameraInfo>, CaptureError> {
    let cams = nokhwa::query(ApiBackend::Auto).map_err(|e| CaptureError::Backend(e.to_string()))?;
    Ok(cams
        .into_iter()
        .map(|c| CameraInfo {
            index: c.index().to_string(),
            name: c.human_name(),
            description: c.description().to_string(),
        })
        .collect())
}

/// A UVC camera opened through nokhwa, converted to grayscale.
pub struct CameraSource {
    camera: Camera,
    started: Instant,
}

impl CameraSource {
    /// Opens the first camera whose name contains `name_contains`
    /// (case-insensitive), at its highest frame rate.
    pub fn open_by_name(name_contains: &str) -> Result<Self, CaptureError> {
        let needle = name_contains.to_lowercase();
        let info = nokhwa::query(ApiBackend::Auto)
            .map_err(|e| CaptureError::Backend(e.to_string()))?
            .into_iter()
            .find(|c| c.human_name().to_lowercase().contains(&needle))
            .ok_or_else(|| CaptureError::NotFound(name_contains.to_string()))?;
        Self::open(info.index().clone())
    }

    /// Opens a camera by index.
    pub fn open(index: CameraIndex) -> Result<Self, CaptureError> {
        let format =
            RequestedFormat::new::<LumaFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        let mut camera =
            Camera::new(index, format).map_err(|e| CaptureError::Backend(e.to_string()))?;
        camera
            .open_stream()
            .map_err(|e| CaptureError::Backend(e.to_string()))?;
        Ok(Self {
            camera,
            started: Instant::now(),
        })
    }
}

impl FrameSource for CameraSource {
    fn next_frame(&mut self) -> Result<Frame, CaptureError> {
        let buf = self
            .camera
            .frame()
            .map_err(|e| CaptureError::Backend(e.to_string()))?;
        let img = buf
            .decode_image::<LumaFormat>()
            .map_err(|e| CaptureError::Backend(e.to_string()))?;
        let (w, h) = (img.width(), img.height());
        let ts = self.started.elapsed().as_micros() as u64;
        Frame::new(w, h, img.into_raw(), ts)
            .ok_or_else(|| CaptureError::Backend("frame size mismatch".into()))
    }
}
