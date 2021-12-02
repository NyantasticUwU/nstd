pub mod key;
pub mod mouse;
pub mod touch;
use key::NSTDKey;
use std::os::raw::c_int;
use winit_input_helper::WinitInputHelper;
/// Left mouse button constant.
pub const NSTD_STD_INPUT_LEFT_MOUSE_BUTTON: usize = 0;
/// Right mouse button constant.
pub const NSTD_STD_INPUT_RIGHT_MOUSE_BUTTON: usize = 1;
/// Middle mouse button constant.
pub const NSTD_STD_INPUT_MIDDLE_MOUSE_BUTTON: usize = 2;

/// A raw input handle.
pub type NSTDRawInput = *mut WinitInputHelper;

/// Checks if a key is down.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDKey key` - The key to check.
/// Returns: `int is_down` - 1 if the key is down, 0 otherwise.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_input_is_key_down(
    raw_input: NSTDRawInput,
    key: NSTDKey,
) -> c_int {
    match key.try_into() {
        Ok(key) => (*raw_input).key_held(key) as c_int,
        _ => 0,
    }
}

/// Checks if a key is up.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDKey key` - The key to check.
/// Returns: `int is_up` - 1 if the key is up, 0 otherwise.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_input_is_key_up(raw_input: NSTDRawInput, key: NSTDKey) -> c_int {
    (nstd_std_input_is_key_down(raw_input, key) == 0) as c_int
}

/// Checks if a mouse button is down.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDUSize button` - The mouse button to check.
/// Returns: `int is_down` - 1 if the mouse button is up, 0 otherwise.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_input_is_mouse_down(
    raw_input: NSTDRawInput,
    button: usize,
) -> c_int {
    (*raw_input).mouse_held(button) as c_int
}

/// Checks if a mouse button is up.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDUSize button` - The mouse button to check.
/// Returns: `int is_up` - 1 if the mouse button is up, 0 otherwise.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_input_is_mouse_up(
    raw_input: NSTDRawInput,
    button: usize,
) -> c_int {
    (nstd_std_input_is_mouse_down(raw_input, button) == 0) as c_int
}
