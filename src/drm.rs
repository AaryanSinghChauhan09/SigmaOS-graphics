//! DRM/KMS model: connectors, CRTCs, encoders, modes, planes.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorType { HDMI, DisplayPort, VGA, Embedded, Virtual }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorStatus { Connected, Disconnected, Unknown }

#[derive(Debug, Clone)]
pub struct Connector {
    pub id: u32,
    pub name: String,
    pub ctype: ConnectorType,
    pub status: ConnectorStatus,
    pub modes: Vec<Mode>,
}

#[derive(Debug, Clone)]
pub struct Mode {
    pub clock: u32,
    pub hdisplay: u16,
    pub vdisplay: u16,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub htotal: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub vtotal: u16,
}

#[derive(Debug, Clone)]
pub struct Crtc {
    pub id: u32,
    pub mode: Option<Mode>,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct Encoder {
    pub id: u32,
    pub encoder_type: u32,
    pub possible_crtcs: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaneType { Primary, Overlay, Cursor }

#[derive(Debug, Clone)]
pub struct Plane {
    pub id: u32,
    pub plane_type: PlaneType,
    pub possible_crtcs: u32,
}

#[derive(Debug, Default)]
pub struct DrmDevice {
    pub connectors: Vec<Connector>,
    pub crtcs: Vec<Crtc>,
    pub encoders: Vec<Encoder>,
    pub planes: Vec<Plane>,
}

impl DrmDevice {
    pub fn new() -> Self { Self::default() }
    pub fn add_connector(&mut self, c: Connector) { self.connectors.push(c); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connector_mode() {
        let c = Connector {
            id: 1,
            name: "HDMI-A-1".into(),
            ctype: ConnectorType::HDMI,
            status: ConnectorStatus::Connected,
            modes: vec![Mode { clock: 148500, hdisplay: 1920, vdisplay: 1080, hsync_start: 2008, hsync_end: 2052, htotal: 2200, vsync_start: 1084, vsync_end: 1089, vtotal: 1125 }],
        };
        assert_eq!(c.modes[0].hdisplay, 1920);
    }
}
