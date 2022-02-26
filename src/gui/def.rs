use winit::monitor::MonitorHandle;

/// Represents a display handle.
pub type NSTDDisplay = *mut MonitorHandle;

/// Represents a window's position.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NSTDWindowPosition {
    /// The position on the x-axis.
    pub x: i32,
    /// The position on the y-axis.
    pub y: i32,
}
impl NSTDWindowPosition {
    /// Creates a new `NSTDWindowPosition` object.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Represents a window's size.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NSTDWindowSize {
    /// The width.
    pub width: u32,
    /// The height.
    pub height: u32,
}
impl NSTDWindowSize {
    /// Creates a new `NSTDWindowSize` object.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
