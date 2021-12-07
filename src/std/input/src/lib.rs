pub mod key;
pub mod mouse;
pub mod touch;
use key::NSTDKey;
use mouse::NSTDMouseButton;
use std::os::raw::c_int;
use winit_input_helper::WinitInputHelper;
#[cfg(feature = "deps")]
pub mod deps {
    pub use num_derive;
    pub use num_traits;
    pub use winit;
    pub use winit_input_helper;
}

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
///     `const NSTDMouseButton button` - The mouse button to check.
/// Returns: `int is_down` - 1 if the mouse button is up, 0 otherwise.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_input_is_mouse_down(
    raw_input: NSTDRawInput,
    button: NSTDMouseButton,
) -> c_int {
    (*raw_input).mouse_held(button as usize) as c_int
}

/// Checks if a mouse button is up.
/// Parameters:
///     `const NSTDRawInput raw_input` - Raw input handle.
///     `const NSTDMouseButton button` - The mouse button to check.
/// Returns: `int is_up` - 1 if the mouse button is up, 0 otherwise.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_input_is_mouse_up(
    raw_input: NSTDRawInput,
    button: NSTDMouseButton,
) -> c_int {
    (nstd_std_input_is_mouse_down(raw_input, button) == 0) as c_int
}
