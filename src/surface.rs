//! Surface and buffer model.

#[derive(Debug, Clone)]
pub struct Buffer {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

#[derive(Debug, Clone)]
pub struct Surface {
    pub buffers: Vec<Buffer>,
    pub width: u32,
    pub height: u32,
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Self {
        Self { buffers: vec![], width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_dimensions() {
        let s = Surface::new(1280, 720);
        assert_eq!(s.width, 1280);
    }
}
