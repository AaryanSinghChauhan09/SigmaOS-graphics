//! SigmaOS-graphics — graphics subsystem model.
//!
//! Models a graphics stack: DRM/KMS (mode setting, connectors, CRTCs, planes),
//! compositor window management, framebuffers, and basic rendering.
//! Deterministic and std-only (no actual hardware access).

pub mod drm;
pub mod compositor;
pub mod framebuffer;
pub mod surface;
pub mod render;

pub use drm::{Connector, Crtc, Encoder, Mode, Plane, DrmDevice};
pub use compositor::{Window, Compositor, Layer};
pub use framebuffer::{Framebuffer, PixelFormat};
pub use surface::{Surface, Buffer};
pub use render::{Renderer, RgbaColor};
