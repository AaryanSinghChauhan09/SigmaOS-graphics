//! Framebuffer model: pixel formats, buffers.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat { Xrgb8888, Argb8888, Rgb565 }

#[derive(Debug, Clone)]
pub struct Framebuffer {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub format: PixelFormat,
    pub handle: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn framebuffer_format() {
        let fb = Framebuffer { id: 1, width: 1920, height: 1080, pitch: 7680, format: PixelFormat::Xrgb8888, handle: 0 };
        assert_eq!(fb.format, PixelFormat::Xrgb8888);
    }
}
