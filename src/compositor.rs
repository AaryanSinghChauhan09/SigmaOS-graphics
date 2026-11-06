//! Compositor model: windows, layers, stacking.

#[derive(Debug, Clone)]
pub struct Window {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer { Background, Bottom, Top, Overlay }

#[derive(Debug, Default)]
pub struct Compositor {
    pub windows: Vec<Window>,
}

impl Compositor {
    pub fn new() -> Self { Self::default() }
    pub fn add_window(&mut self, w: Window) { self.windows.push(w); }
    pub fn remove_window(&mut self, id: u32) {
        self.windows.retain(|w| w.id != id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_manager() {
        let mut c = Compositor::new();
        c.add_window(Window { id: 1, x: 0, y: 0, width: 800, height: 600, visible: true });
        assert_eq!(c.windows.len(), 1);
    }
}
