//! A display/monitor handle.
use super::def::NSTDWindowSize;
use winit::monitor::MonitorHandle;

/// Represents a display handle.
pub type NSTDDisplay = *mut MonitorHandle;

/// Returns a display's size.
///
/// # Parameters
///
/// - `const NSTDDisplay display` - The display.
///
/// # Returns
///
/// `NSTDWindowSize size` - The size of the display.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_display_get_size(display: NSTDDisplay) -> NSTDWindowSize {
    let size = (*display).size();
    NSTDWindowSize::new(size.width, size.height)
}

/// Returns the display's scale factor.
///
/// # Parameters
///
/// - `const NSTDDisplay display` - The display.
///
/// # Returns
///
/// `NSTDFloat64 scale_factor` - The scale factor of the display.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_display_get_scale_factor(display: NSTDDisplay) -> f64 {
    (*display).scale_factor()
}

/// Frees a display handle.
///
/// # Parameters
///
/// - `NSTDDisplay *const display` - Pointer to the display handle.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_display_free(display: *mut NSTDDisplay) {
    Box::from_raw(*display);
    *display = std::ptr::null_mut();
}
