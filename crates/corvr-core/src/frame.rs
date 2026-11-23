/// A single 8-bit grayscale camera image.
///
/// Both the Beyond 2e eye cameras and the Babble mouth camera are IR cameras,
/// so trackers work on grayscale only.
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    /// Row-major pixels, `width * height` bytes.
    pub pixels: Vec<u8>,
    /// Capture time in microseconds since the source started.
    pub timestamp_us: u64,
}

impl Frame {
    /// Creates a frame, checking that the buffer matches the size.
    pub fn new(width: u32, height: u32, pixels: Vec<u8>, timestamp_us: u64) -> Option<Self> {
        (pixels.len() == width as usize * height as usize).then_some(Self {
            width,
            height,
            pixels,
            timestamp_us,
        })
    }

    /// Pixel at `(x, y)`, or `None` outside the image.
    pub fn get(&self, x: u32, y: u32) -> Option<u8> {
        (x < self.width && y < self.height)
            .then(|| self.pixels[y as usize * self.width as usize + x as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_wrong_buffer_size() {
        assert!(Frame::new(2, 2, vec![0; 3], 0).is_none());
        let f = Frame::new(2, 2, vec![1, 2, 3, 4], 0).unwrap();
        assert_eq!(f.get(1, 1), Some(4));
        assert_eq!(f.get(2, 0), None);
    }
}
