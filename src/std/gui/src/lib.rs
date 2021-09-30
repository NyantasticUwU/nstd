use nstd_events::NSTDEventLoop;
use std::{
    ffi::CStr,
    os::raw::{c_char, c_double, c_int, c_void},
    ptr, slice,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::EventLoop,
    window::Window,
};

/// Represents a window.
type NSTDWindow = *mut c_void;

/// Creates a new window.
/// Parameters:
///     `NSTDEventLoop event_loop` - The event loop to attach to the window.
/// Returns: `NSTDWindow window` - The new window, null on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_create(event_loop: NSTDEventLoop) -> NSTDWindow {
    let event_loop = &*(event_loop as *mut EventLoop<()>);
    match Window::new(event_loop) {
        Ok(window) => Box::into_raw(Box::new(window)) as NSTDWindow,
        _ => ptr::null_mut(),
    }
}

/// Gets a window's scale factor.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `double factor` - The scale factor of the window.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_get_scale_factor(window: NSTDWindow) -> c_double {
    let window = &*(window as *mut Window);
    window.scale_factor()
}

/// Sets a window's position.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDInt32 *const pos` - An array of 2 `NSTDInt32`s.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_position(window: NSTDWindow, pos: *const i32) {
    let window = &*(window as *mut Window);
    let pos = slice::from_raw_parts(pos, 2);
    window.set_outer_position(PhysicalPosition::new(pos[0], pos[1]));
}

/// Gets a window's position.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDInt32 *pos` - An array of 2 `NSTDInt32`s.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_get_position(
    window: NSTDWindow,
    pos: *mut i32,
) -> c_int {
    let window = &*(window as *mut Window);
    match window.outer_position() {
        Ok(outer_size) => {
            let s = slice::from_raw_parts_mut(pos, 2);
            s[0] = outer_size.x;
            s[1] = outer_size.y;
            0
        }
        _ => 1,
    }
}

/// Gets a window's client position.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDInt32 *pos` - An array of 2 `NSTDInt32`s.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_get_client_position(
    window: NSTDWindow,
    pos: *mut i32,
) -> c_int {
    let window = &*(window as *mut Window);
    match window.inner_position() {
        Ok(inner_size) => {
            let s = slice::from_raw_parts_mut(pos, 2);
            s[0] = inner_size.x;
            s[1] = inner_size.y;
            0
        }
        _ => 1,
    }
}

/// Gets a window's size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDUInt32 *size` - An array of 2 `NSTDInt32`s.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_get_size(window: NSTDWindow, size: *mut u32) {
    let window = &*(window as *mut Window);
    let outer_size = window.outer_size();
    let s = slice::from_raw_parts_mut(size, 2);
    s[0] = outer_size.width;
    s[1] = outer_size.height;
}

/// Sets a window's client size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDUInt32 *const size` - An array of 2 `NSTDInt32`s.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_client_size(window: NSTDWindow, size: *const u32) {
    let window = &*(window as *mut Window);
    let s = slice::from_raw_parts(size, 2);
    window.set_inner_size(PhysicalSize::new(s[0], s[1]));
}

/// Gets a window's client size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDUInt32 *size` - An array of 2 `NSTDInt32`s.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_get_client_size(window: NSTDWindow, size: *mut u32) {
    let window = &*(window as *mut Window);
    let outer_size = window.inner_size();
    let s = slice::from_raw_parts_mut(size, 2);
    s[0] = outer_size.width;
    s[1] = outer_size.height;
}

/// Sets a window's client min size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDUInt32 *size` - An array of 2 `NSTDUInt32`s, null for no min.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_client_min_size(
    window: NSTDWindow,
    size: *const u32,
) {
    let window = &*(window as *mut Window);
    if !size.is_null() {
        let s = slice::from_raw_parts(size, 2);
        window.set_min_inner_size(Some(PhysicalSize::new(s[0], s[1])));
    } else {
        window.set_min_inner_size::<PhysicalSize<u32>>(None);
    }
}

/// Sets a window's client max size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDUInt32 *size` - An array of 2 `NSTDUInt32`s, null for no max.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_client_max_size(
    window: NSTDWindow,
    size: *const u32,
) {
    let window = &*(window as *mut Window);
    if !size.is_null() {
        let s = slice::from_raw_parts(size, 2);
        window.set_max_inner_size(Some(PhysicalSize::new(s[0], s[1])));
    } else {
        window.set_max_inner_size::<PhysicalSize<u32>>(None);
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
            let window = &*(window as *mut Window);
            window.set_title(title);
            0
        }
        _ => 1,
    }
}

/// Sets a window's visibility.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int visible` - Whether to show or hide the window.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_visible(window: NSTDWindow, visible: c_int) {
    let window = &*(window as *mut Window);
    window.set_visible(visible != 0);
}

/// Sets whether the window is resizable or not.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int resizable` - Whether the window should be resizable or not.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_resizable(window: NSTDWindow, resizable: c_int) {
    let window = &*(window as *mut Window);
    window.set_resizable(resizable != 0);
}

/// Sets the window's minimization mode.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int minimized` - Whether the window should be minimized or not.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_minimized(window: NSTDWindow, minimized: c_int) {
    let window = &*(window as *mut Window);
    window.set_minimized(minimized != 0);
}

/// Sets the window's maximization mode.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int maximized` - Whether the window should be maximized or not.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_maximized(window: NSTDWindow, maximized: c_int) {
    let window = &*(window as *mut Window);
    window.set_maximized(maximized != 0);
}

/// Checks if the window is maximized.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `int maximized` - Nonzero if the window is maximized.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_is_maximized(window: NSTDWindow) -> c_int {
    let window = &*(window as *mut Window);
    window.is_maximized() as c_int
}

/// Turn window decorations on or off.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int decorations` - Whether to allow window decorations or not.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gui_window_set_decorations(
    window: NSTDWindow,
    decorations: c_int,
) {
    let window = &*(window as *mut Window);
    window.set_decorations(decorations != 0);
}
