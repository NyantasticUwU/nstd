use crate::core::def::NSTDBool;
use winit::window::WindowId;

/// Represents a window ID.
pub type NSTDWindowID = *mut WindowId;

/// Compares two window IDs.
/// Parameters:
///     `const NSTDWindowID id1` - A window ID.
///     `const NSTDWindowID id2` - Another window ID.
/// Returns: `NSTDBool are_same` - 1 if the two IDs refer to the same window, 0 otherwise.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_id_compare(
    id1: NSTDWindowID,
    id2: NSTDWindowID,
) -> NSTDBool {
    (*id1 == *id2).into()
}

/// Frees a window ID.
/// Parameters:
///     `NSTDWindowID *const window_id` - Pointer to the window ID.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_id_free(window_id: *mut NSTDWindowID) {
    Box::from_raw(*window_id);
    *window_id = std::ptr::null_mut();
}
