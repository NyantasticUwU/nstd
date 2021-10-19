use nstd_events::NSTDEventLoop;
use std::{
    ffi::CStr,
    os::raw::{c_char, c_double, c_int},
    ptr, slice,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    monitor::MonitorHandle,
    window::{Icon, Window},
};

/// Represents a window.
pub type NSTDWindow = *mut Window;

/// Represents a display handle.
pub type NSTDDisplay = *mut MonitorHandle;

/// Represents a window's position.
#[repr(C)]
pub struct NSTDWindowPosition {
    pub x: i32,
    pub y: i32,
}

/// Represents a window's size.
#[repr(C)]
pub struct NSTDWindowSize {
    pub width: u32,
    pub height: u32,
}
impl NSTDWindowSize {
    /// Creates a new `NSTDWindowSize` object.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

/// Creates a new window.
/// Parameters:
///     `NSTDEventLoop event_loop` - The event loop to attach to the window.
/// Returns: `NSTDWindow window` - The new window, null on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_create(event_loop: NSTDEventLoop) -> NSTDWindow {
    match Window::new(&*event_loop) {
        Ok(window) => Box::into_raw(Box::new(window)),
        _ => ptr::null_mut(),
    }
}

/// Gets a window's scale factor.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `double factor` - The scale factor of the window.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_get_scale_factor(window: NSTDWindow) -> c_double {
    (*window).scale_factor()
}

/// Sets a window's position.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDWindowPosition pos` - The new position.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_position(
    window: NSTDWindow,
    pos: NSTDWindowPosition,
) {
    (*window).set_outer_position(PhysicalPosition::new(pos.x, pos.y));
}

/// Gets a window's position.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDWindowPosition *pos` - Returns as the position.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_get_position(
    window: NSTDWindow,
    pos: *mut NSTDWindowPosition,
) -> c_int {
    match (*window).outer_position() {
        Ok(outer_size) => {
            let pos = &mut *pos;
            pos.x = outer_size.x;
            pos.y = outer_size.y;
            0
        }
        _ => 1,
    }
}

/// Gets a window's client position.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDWindowPosition *pos` - Returns as the position.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_get_client_position(
    window: NSTDWindow,
    pos: *mut NSTDWindowPosition,
) -> c_int {
    match (*window).inner_position() {
        Ok(inner_size) => {
            let pos = &mut *pos;
            pos.x = inner_size.x;
            pos.y = inner_size.y;
            0
        }
        _ => 1,
    }
}

/// Gets a window's size.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `NSTDWindowSize size` - The size of the window.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_get_size(window: NSTDWindow) -> NSTDWindowSize {
    let size = (*window).outer_size();
    NSTDWindowSize::new(size.width, size.height)
}

/// Sets a window's client size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDWindowSize size` - An array of 2 `NSTDInt32`s.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_client_size(
    window: NSTDWindow,
    size: NSTDWindowSize,
) {
    (*window).set_inner_size(PhysicalSize::new(size.width, size.height));
}

/// Gets a window's client size.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `NSTDWindowSize size` - The size of the window's client area.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_get_client_size(window: NSTDWindow) -> NSTDWindowSize {
    let size = (*window).inner_size();
    NSTDWindowSize::new(size.width, size.height)
}

/// Sets a window's client min size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDWindowSize *const size` - An array of 2 `NSTDUInt32`s, null for no min.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_client_min_size(
    window: NSTDWindow,
    size: *const NSTDWindowSize,
) {
    if !size.is_null() {
        let size = &*size;
        (*window).set_min_inner_size(Some(PhysicalSize::new(size.width, size.height)));
    } else {
        (*window).set_min_inner_size::<PhysicalSize<u32>>(None);
    }
}

/// Sets a window's client max size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDWindowSize *const size` - An array of 2 `NSTDUInt32`s, null for no max.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_client_max_size(
    window: NSTDWindow,
    size: *const NSTDWindowSize,
) {
    if !size.is_null() {
        let size = &*size;
        (*window).set_max_inner_size(Some(PhysicalSize::new(size.width, size.height)));
    } else {
        (*window).set_max_inner_size::<PhysicalSize<u32>>(None);
    }
}

/// Sets a window's title.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const char *const title` - The new window title.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_title(
    window: NSTDWindow,
    title: *const c_char,
) -> c_int {
    match CStr::from_ptr(title).to_str() {
        Ok(title) => {
            (*window).set_title(title);
            0
        }
        _ => 1,
    }
}

/// Sets a window's visibility.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int visible` - Whether to show or hide the window.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_visible(window: NSTDWindow, visible: c_int) {
    (*window).set_visible(visible != 0);
}

/// Sets whether the window is resizable or not.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int resizable` - Whether the window should be resizable or not.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_resizable(window: NSTDWindow, resizable: c_int) {
    (*window).set_resizable(resizable != 0);
}

/// Sets the window's minimization mode.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int minimized` - Whether the window should be minimized or not.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_minimized(window: NSTDWindow, minimized: c_int) {
    (*window).set_minimized(minimized != 0);
}

/// Sets the window's maximization mode.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int maximized` - Whether the window should be maximized or not.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_maximized(window: NSTDWindow, maximized: c_int) {
    (*window).set_maximized(maximized != 0);
}

/// Checks if the window is maximized.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `int maximized` - Nonzero if the window is maximized.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_is_maximized(window: NSTDWindow) -> c_int {
    (*window).is_maximized() as c_int
}

/// Sets a window's icon image.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDByte *const raw` - Raw image data, must be in RGBA format or null.
///     `const NSTDUInt32 width` - The image width.
///     `const NSTDUInt32 height` - The image height.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_icon(
    window: NSTDWindow,
    raw: *const u8,
    width: u32,
    height: u32,
) -> c_int {
    let icon = match !raw.is_null() {
        true => {
            const RGBA_COMPONENTS: u32 = 4;
            let raw = slice::from_raw_parts(raw, (RGBA_COMPONENTS * width * height) as usize);
            match Icon::from_rgba(raw.to_vec(), width, height) {
                Ok(icon) => Some(icon),
                _ => return 1,
            }
        }
        false => None,
    };
    (*window).set_window_icon(icon);
    0
}

/// Turn window decorations on or off.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int decorations` - Whether to allow window decorations or not.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_decorations(
    window: NSTDWindow,
    decorations: c_int,
) {
    (*window).set_decorations(decorations != 0);
}

/// Gets the display that the given window resides in.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `NSTDDisplay display` - The display that the window is in.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_get_display(window: NSTDWindow) -> NSTDDisplay {
    match (*window).current_monitor() {
        Some(handle) => Box::into_raw(Box::new(handle)),
        _ => ptr::null_mut(),
    }
}

/// Closes a window.
/// Parameters:
///     `NSTDWindow *window` - Pointer to the window.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_close(window: *mut NSTDWindow) {
    Box::from_raw(*window);
    *window = ptr::null_mut();
}

/// Returns a display's size.
/// Parameters:
///     `NSTDDisplay display` - The display.
/// Returns: `NSTDWindowSize size` - The size of the display.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_display_get_size(display: NSTDDisplay) -> NSTDWindowSize {
    let size = (*display).size();
    NSTDWindowSize::new(size.width, size.height)
}

/// Returns the display's scale factor.
/// Parameters:
///     `NSTDDisplay display` - The display.
/// Returns: `double scale_factor` - The scale factor of the display.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_display_get_scale_factor(display: NSTDDisplay) -> c_double {
    (*display).scale_factor()
}

/// Frees a display handle.
/// Parameters:
///     `NSTDDisplay *display` - Pointer to the display handle.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_display_free(display: *mut NSTDDisplay) {
    Box::from_raw(*display);
    *display = ptr::null_mut();
}
