use nstd_events::NSTDEventLoop;
use std::{
    ffi::CStr,
    os::raw::{c_char, c_double, c_int, c_void},
    ptr,
};
use winit::{event_loop::EventLoop, window::Window};

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
