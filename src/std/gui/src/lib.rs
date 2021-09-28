use nstd_events::NSTDEventLoop;
use std::{
    ffi::CStr,
    os::raw::{c_char, c_int, c_void},
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
