//! Keyboard/Mouse input.
pub mod key;
pub mod mouse;
use self::{key::NSTDKey, mouse::NSTDMouseButton};
use crate::core::def::NSTDBool;
use winit_input_helper::WinitInputHelper;

/// A raw input handle.
pub type NSTDRawInput = *mut WinitInputHelper;

/// Checks if a key is down.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDKey key` - The key to check.
/// Returns: `NSTDBool is_down` - `NSTD_BOOL_TRUE` if the key is down.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_input_is_key_down(raw_input: NSTDRawInput, key: NSTDKey) -> NSTDBool {
    if let Ok(key) = key.try_into() {
        return NSTDBool::from((*raw_input).key_held(key));
    }
    NSTDBool::NSTD_BOOL_FALSE
}

/// Checks if a key is up.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDKey key` - The key to check.
/// Returns: `NSTDBool is_up` - `NSTD_BOOL_TRUE` if the key is up.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_input_is_key_up(raw_input: NSTDRawInput, key: NSTDKey) -> NSTDBool {
    NSTDBool::from(nstd_input_is_key_down(raw_input, key) == NSTDBool::NSTD_BOOL_FALSE)
}

/// Checks if a mouse button is down.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDMouseButton button` - The mouse button to check.
/// Returns: `NSTDBool is_down` - `NSTD_BOOL_TRUE` if the mouse button is up.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_input_is_mouse_down(
    raw_input: NSTDRawInput,
    button: NSTDMouseButton,
) -> NSTDBool {
    NSTDBool::from((*raw_input).mouse_held(button as usize))
}

/// Checks if a mouse button is up.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDMouseButton button` - The mouse button to check.
/// Returns: `NSTDBool is_up` - `NSTD_BOOL_TRUE` if the mouse button is up.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_input_is_mouse_up(
    raw_input: NSTDRawInput,
    button: NSTDMouseButton,
) -> NSTDBool {
    NSTDBool::from(nstd_input_is_mouse_down(raw_input, button) == NSTDBool::NSTD_BOOL_FALSE)
}
