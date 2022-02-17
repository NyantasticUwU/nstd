use crate::{
    core::{
        def::{NSTDBool, NSTDErrorCode},
        str::NSTDStr,
    },
    events::{NSTDEventLoop, NSTDWindowID},
    image::NSTDImage,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    monitor::MonitorHandle,
    window::{Icon, Window},
};
#[cfg(target_os = "windows")]
use winit::{
    platform::windows::{WindowBuilderExtWindows, WindowExtWindows},
    window::WindowBuilder,
};

/// Represents a window.
pub type NSTDWindow = *mut Window;

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

/// Creates a new window.
/// Parameters:
///     `const NSTDEventLoop event_loop` - The event loop to attach to the window.
/// Returns: `NSTDWindow window` - The new window, null on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_create(event_loop: NSTDEventLoop) -> NSTDWindow {
    match Window::new(&*event_loop) {
        Ok(window) => Box::into_raw(Box::new(window)),
        _ => std::ptr::null_mut(),
    }
}

/// Creates a child window with `parent` being the parent window.
/// NOTE: This is only functional on Windows targets and will always return a null window handle on
/// any other platform.
/// Parameters:
///     `const NSTDEventLoop event_loop` - The event loop to attach to the window.
///     `const NSTDWindow parent` - The parent window.
/// Returns: `NSTDWindow child` - The new child window.
#[cfg_attr(feature = "clib", no_mangle)]
#[cfg_attr(not(target_os = "windows"), allow(unused_variables))]
pub unsafe extern "C" fn nstd_gui_window_create_child(
    event_loop: NSTDEventLoop,
    parent: NSTDWindow,
) -> NSTDWindow {
    #[cfg(target_os = "windows")]
    {
        let parent = (*parent).hwnd().cast();
        let window = WindowBuilder::new().with_parent_window(parent);
        match window.build(&*event_loop) {
            Ok(window) => Box::into_raw(Box::new(window)),
            _ => std::ptr::null_mut(),
        }
    }
    #[cfg(not(target_os = "windows"))]
    std::ptr::null_mut()
}

/// Requests the window to be drawn.
/// Parameters:
///     `const NSTDWindow window` - The window.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_request_redraw(window: NSTDWindow) {
    (*window).request_redraw();
}

/// Gets a window's scale factor.
/// Parameters:
///     `const NSTDWindow window` - The window.
/// Returns: `NSTDFloat64 factor` - The scale factor of the window.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_get_scale_factor(window: NSTDWindow) -> f64 {
    (*window).scale_factor()
}

/// Sets a window's position.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDWindowPosition pos` - The new position.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_set_position(window: NSTDWindow, pos: NSTDWindowPosition) {
    (*window).set_outer_position(PhysicalPosition::new(pos.x, pos.y));
}

/// Gets a window's position.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `NSTDWindowPosition *const pos` - Returns as the position.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_get_position(
    window: NSTDWindow,
    pos: *mut NSTDWindowPosition,
) -> NSTDErrorCode {
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
///     `const NSTDWindow window` - The window.
///     `NSTDWindowPosition *const pos` - Returns as the position.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_get_client_position(
    window: NSTDWindow,
    pos: *mut NSTDWindowPosition,
) -> NSTDErrorCode {
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
///     `const NSTDWindow window` - The window.
/// Returns: `NSTDWindowSize size` - The size of the window.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_get_size(window: NSTDWindow) -> NSTDWindowSize {
    let size = (*window).outer_size();
    NSTDWindowSize::new(size.width, size.height)
}

/// Sets a window's client size.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDWindowSize size` - An array of 2 `NSTDInt32`s.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_set_client_size(window: NSTDWindow, size: NSTDWindowSize) {
    (*window).set_inner_size(PhysicalSize::new(size.width, size.height));
}

/// Gets a window's client size.
/// Parameters:
///     `const NSTDWindow window` - The window.
/// Returns: `NSTDWindowSize size` - The size of the window's client area.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_get_client_size(window: NSTDWindow) -> NSTDWindowSize {
    let size = (*window).inner_size();
    NSTDWindowSize::new(size.width, size.height)
}

/// Sets a window's client min size.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDWindowSize *const size` - An array of 2 `NSTDUInt32`s, null for no min.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_set_client_min_size(
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
///     `const NSTDWindow window` - The window.
///     `const NSTDWindowSize *const size` - An array of 2 `NSTDUInt32`s, null for no max.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_set_client_max_size(
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
///     `const NSTDWindow window` - The window.
///     `const NSTDStr *const title` - The new window title.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_set_title(
    window: NSTDWindow,
    title: &NSTDStr,
) -> NSTDErrorCode {
    match std::str::from_utf8(title.bytes.as_byte_slice()) {
        Ok(title) => {
            (*window).set_title(title);
            0
        }
        _ => 1,
    }
}

/// Sets a window's visibility.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDBool visible` - Whether to show or hide the window.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_set_visible(window: NSTDWindow, visible: NSTDBool) {
    (*window).set_visible(visible.into());
}

/// Sets whether the window is resizable or not.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDBool resizable` - Whether the window should be resizable or not.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_set_resizable(window: NSTDWindow, resizable: NSTDBool) {
    (*window).set_resizable(resizable.into());
}

/// Sets the window's minimization mode.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDBool minimized` - Whether the window should be minimized or not.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_set_minimized(window: NSTDWindow, minimized: NSTDBool) {
    (*window).set_minimized(minimized.into());
}

/// Sets the window's maximization mode.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDBool maximized` - Whether the window should be maximized or not.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_set_maximized(window: NSTDWindow, maximized: NSTDBool) {
    (*window).set_maximized(maximized.into());
}

/// Checks if the window is maximized.
/// Parameters:
///     `const NSTDWindow window` - The window.
/// Returns: `NSTDBool maximized` - Nonzero if the window is maximized.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_is_maximized(window: NSTDWindow) -> NSTDBool {
    (*window).is_maximized().into()
}

/// Sets a window's icon image.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDImage *const img` - The icon image, null for default.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_set_icon(
    window: NSTDWindow,
    img: *const NSTDImage,
) -> NSTDErrorCode {
    let icon = match !img.is_null() {
        true => {
            const RGBA_COMPONENTS: u32 = 4;
            let size = (RGBA_COMPONENTS * (*img).width * (*img).height) as usize;
            let raw = std::slice::from_raw_parts((*img).raw, size);
            match Icon::from_rgba(raw.to_vec(), (*img).width, (*img).height) {
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
///     `const NSTDWindow window` - The window.
///     `const NSTDBool decorations` - Whether to allow window decorations or not.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_set_decorations(
    window: NSTDWindow,
    decorations: NSTDBool,
) {
    (*window).set_decorations(decorations.into());
}

/// Gets the window's ID.
/// Parameters:
///     `const NSTDWindow window` - The window.
/// Returns: `NSTDWindowID window_id` - The window ID.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_get_id(window: NSTDWindow) -> NSTDWindowID {
    Box::into_raw(Box::new((*window).id()))
}

/// Gets the display that the given window resides in.
/// Parameters:
///     `const NSTDWindow window` - The window.
/// Returns: `NSTDDisplay display` - The display that the window is in.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_get_display(window: NSTDWindow) -> NSTDDisplay {
    match (*window).current_monitor() {
        Some(handle) => Box::into_raw(Box::new(handle)),
        _ => std::ptr::null_mut(),
    }
}

/// Closes a window.
/// Parameters:
///     `NSTDWindow *const window` - Pointer to the window.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_window_close(window: *mut NSTDWindow) {
    Box::from_raw(*window);
    *window = std::ptr::null_mut();
}

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

/// Returns a display's size.
/// Parameters:
///     `const NSTDDisplay display` - The display.
/// Returns: `NSTDWindowSize size` - The size of the display.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_display_get_size(display: NSTDDisplay) -> NSTDWindowSize {
    let size = (*display).size();
    NSTDWindowSize::new(size.width, size.height)
}

/// Returns the display's scale factor.
/// Parameters:
///     `const NSTDDisplay display` - The display.
/// Returns: `NSTDFloat64 scale_factor` - The scale factor of the display.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_display_get_scale_factor(display: NSTDDisplay) -> f64 {
    (*display).scale_factor()
}

/// Frees a display handle.
/// Parameters:
///     `NSTDDisplay *const display` - Pointer to the display handle.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gui_display_free(display: *mut NSTDDisplay) {
    Box::from_raw(*display);
    *display = std::ptr::null_mut();
}
