use wgpu::{Surface, SurfaceConfiguration};

/// Represents a graphical surface.
pub type NSTDGLSurface = *mut Surface;

/// Represents a surface config.
pub type NSTDGLSurfaceConfiguration = *mut SurfaceConfiguration;
