//! Mouse types.

/// Represents a mouse button state.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDMouseButtonState {
    /// A mouse button is/was released.
    NSTD_MOUSE_BUTTON_STATE_RELEASED,
    /// A mouse button is/was pressed.
    NSTD_MOUSE_BUTTON_STATE_PRESSED,
}
impl Default for NSTDMouseButtonState {
    #[inline]
    fn default() -> Self {
        Self::NSTD_MOUSE_BUTTON_STATE_RELEASED
    }
}

/// Represents a mouse button.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDMouseButton {
    /// The left mouse button.
    NSTD_MOUSE_BUTTON_LEFT,
    /// The right mouse button.
    NSTD_MOUSE_BUTTON_RIGHT,
    /// The middle mouse button.
    NSTD_MOUSE_BUTTON_MIDDLE,
    /// An extra mouse button.
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
    /// The mouse button.
    pub button: NSTDMouseButton,
    /// The index of an extra button.
    pub extra_button: u16,
    /// The state of the mouse button.
    pub state: NSTDMouseButtonState,
}
