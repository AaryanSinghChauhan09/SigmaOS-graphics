//! Basic rendering: RGBA color, simple draw operations.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for RgbaColor {
    fn default() -> Self { Self::rgb(0, 0, 0) }
}

impl RgbaColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r, g, b, a } }
    pub fn rgb(r: u8, g: u8, b: u8) -> Self { Self { r, g, b, a: 255 } }
}

#[derive(Debug, Default)]
pub struct Renderer {
    pub clear_color: RgbaColor,
}

impl Renderer {
    pub fn new() -> Self { Self::default() }
    pub fn set_clear_color(&mut self, c: RgbaColor) { self.clear_color = c; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgba_color() {
        let c = RgbaColor::rgb(255, 0, 0);
        assert_eq!(c.r, 255);
        assert_eq!(c.a, 255);
    }
}
