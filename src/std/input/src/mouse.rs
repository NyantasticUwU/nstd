/// Represents a mouse button state.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDMouseButtonState {
    NSTD_MOUSE_BUTTON_PRESSED,
    NSTD_MOUSE_BUTTON_RELEASED,
}
impl Default for NSTDMouseButtonState {
    #[inline]
    fn default() -> Self {
        Self::NSTD_MOUSE_BUTTON_RELEASED
    }
}

/// Represents a mouse button.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDMouseButton {
    NSTD_MOUSE_BUTTON_LEFT,
    NSTD_MOUSE_BUTTON_RIGHT,
    NSTD_MOUSE_BUTTON_MIDDLE,
    NSTD_MOUSE_BUTTON_OTHER,
}
impl Default for NSTDMouseButton {
    #[inline]
    fn default() -> Self {
        Self::NSTD_MOUSE_BUTTON_OTHER
    }
}

/// Represents a mouse button event.
#[repr(C)]
#[derive(Default)]
pub struct NSTDMouseButtonEvent {
    pub button: NSTDMouseButton,
    pub extra_button: u16,
    pub state: NSTDMouseButtonState,
}
