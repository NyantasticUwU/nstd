use self::NSTDEventLoopControlFlow::*;
use crate::{
    core::{def::NSTDBool, slice::NSTDSlice},
    input::key::{NSTDKey, NSTDKeyEvent, NSTDKeyState},
};
#[cfg(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "macos",
    target_os = "android"
))]
use winit::platform::run_return::EventLoopExtRunReturn;
#[cfg(target_os = "linux")]
use winit::platform::unix::EventLoopExtUnix;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopExtWindows;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::WindowId,
};

/// An event loop handle.
pub type NSTDEventLoop = *mut EventLoop<()>;

/// Represents a window ID.
pub type NSTDWindowID = *mut WindowId;

/// Represents a device ID.
pub type NSTDDeviceID = *const DeviceId;

/// Represents an event loop's control flow.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDEventLoopControlFlow {
    /// Event loop should poll after this iteration.
    NSTD_EVENT_LOOP_CONTROL_FLOW_POLL,
    /// Event loop should wait after this iteration.
    NSTD_EVENT_LOOP_CONTROL_FLOW_WAIT,
    /// Event loop should exit after this iteration.
    NSTD_EVENT_LOOP_CONTROL_FLOW_EXIT,
}
impl Into<ControlFlow> for NSTDEventLoopControlFlow {
    #[inline]
    fn into(self) -> ControlFlow {
        match self {
            Self::NSTD_EVENT_LOOP_CONTROL_FLOW_POLL => ControlFlow::Poll,
            Self::NSTD_EVENT_LOOP_CONTROL_FLOW_WAIT => ControlFlow::Wait,
            Self::NSTD_EVENT_LOOP_CONTROL_FLOW_EXIT => ControlFlow::Exit,
        }
    }
}

/// Event callbacks.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NSTDEventCallbacks {
    /// Called when all main events have been processed.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    pub on_update: Option<unsafe extern "C" fn(&mut NSTDEventLoopControlFlow)>,
    /// Called after a window is resized.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `const NSTDSlice *size` - Two `NSTDUInt32`s representing 'width' and 'height'.
    pub on_window_resized:
        Option<unsafe extern "C" fn(&mut NSTDEventLoopControlFlow, NSTDWindowID, &NSTDSlice)>,
    /// Called after a window is moved.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `const NSTDSlice *size` - Two `NSTDInt32`s representing 'x' and 'y'.
    pub on_window_moved:
        Option<unsafe extern "C" fn(&mut NSTDEventLoopControlFlow, NSTDWindowID, &NSTDSlice)>,
    /// Called when a window's focus changes.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `NSTDBool is_focused` - `NSTD_BOOL_TRUE` if the window gained focus.
    pub on_window_focus_changed:
        Option<unsafe extern "C" fn(&mut NSTDEventLoopControlFlow, NSTDWindowID, NSTDBool)>,
    /// Called when a window recieve's keyboard input.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `NSTDDeviceID device_id` - The device ID of the keyboard.
    ///     `const NSTDKeyEvent *key` - A pointer to the key data.
    pub on_window_keyboard_input: Option<
        unsafe extern "C" fn(
            &mut NSTDEventLoopControlFlow,
            NSTDWindowID,
            NSTDDeviceID,
            &NSTDKeyEvent,
        ),
    >,
    /// Called when a cursor has moved within a window.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `NSTDDeviceID device_id` - The device ID of the cursor.
    ///     `const NSTDSlice *pos` - Two `NSTDFloat64`s representing the cursor's position.
    pub on_window_cursor_moved: Option<
        unsafe extern "C" fn(&mut NSTDEventLoopControlFlow, NSTDWindowID, NSTDDeviceID, &NSTDSlice),
    >,
    /// Called when a cursor enters a window.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `NSTDDeviceID device_id` - The device ID of the cursor.
    pub on_window_cursor_entered:
        Option<unsafe extern "C" fn(&mut NSTDEventLoopControlFlow, NSTDWindowID, NSTDDeviceID)>,
    /// Called when a cursor leaves a window.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `NSTDDeviceID device_id` - The device ID of the cursor.
    pub on_window_cursor_left:
        Option<unsafe extern "C" fn(&mut NSTDEventLoopControlFlow, NSTDWindowID, NSTDDeviceID)>,
    /// Called when a window requests closing.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window that requests closing.
    pub on_window_requests_closing:
        Option<unsafe extern "C" fn(&mut NSTDEventLoopControlFlow, NSTDWindowID)>,
    /// Called when a window is destroyed.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    pub on_window_destroyed:
        Option<unsafe extern "C" fn(&mut NSTDEventLoopControlFlow, NSTDWindowID)>,
    /// Called when the event loop is being destroyed.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    pub on_destroy: Option<unsafe extern "C" fn(&mut NSTDEventLoopControlFlow)>,
}

/// Creates a new event loop.
/// Returns: `NSTDEventLoop event_loop` - The event loop.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_events_event_loop_new() -> NSTDEventLoop {
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    return Box::into_raw(Box::new(EventLoop::new()));
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    return Box::into_raw(Box::new(EventLoop::<()>::new_any_thread()));
}

/// Runs an event loop, never returning.
/// Note that this function returns on the following operating systems:
///     - Windows
///     - Linux
///     - MacOS
///     - Android
/// Parameters:
///     `NSTDEventLoop *const event_loop` - The event loop to run.
///     `const NSTDEventCallbacks *const callbacks` - The event callbacks.
///     `const NSTDBool should_return` - `NSTD_BOOL_TRUE` if this function should return.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_events_event_loop_run(
    event_loop: *mut NSTDEventLoop,
    callbacks: *const NSTDEventCallbacks,
    should_return: NSTDBool,
) {
    // Regaining ownership of the winit event loop.
    let winit_event_loop = *event_loop;
    *event_loop = std::ptr::null_mut();
    let mut event_loop = Box::from_raw(winit_event_loop);
    // Creating the event handler closure.
    let closure = move |event: Event<()>, _: &EventLoopWindowTarget<()>, cf: &mut ControlFlow| {
        let mut ncf = NSTD_EVENT_LOOP_CONTROL_FLOW_POLL;
        event_handler(event, &mut ncf, &*callbacks);
        *cf = ncf.into();
    };
    // Running the event loop.
    #[cfg(not(any(
        target_os = "windows",
        target_os = "linux",
        target_os = "macos",
        target_os = "android"
    )))]
    event_loop.run(closure);
    #[cfg(any(
        target_os = "windows",
        target_os = "linux",
        target_os = "macos",
        target_os = "android"
    ))]
    match should_return {
        NSTDBool::NSTD_BOOL_TRUE => event_loop.run_return(closure),
        NSTDBool::NSTD_BOOL_FALSE => event_loop.run(closure),
    }
}

/// The winit event handler.
#[inline]
unsafe fn event_handler(
    event: Event<()>,
    ncf: &mut NSTDEventLoopControlFlow,
    callbacks: &NSTDEventCallbacks,
) {
    match event {
        // All main events have been processed.
        Event::MainEventsCleared => {
            if let Some(on_update) = callbacks.on_update {
                on_update(ncf);
            }
        }
        // The event loop is being destroyed.
        Event::LoopDestroyed => {
            if let Some(on_destroy) = callbacks.on_destroy {
                on_destroy(ncf);
            }
        }
        Event::WindowEvent {
            event,
            mut window_id,
        } => match event {
            // A window was resized.
            WindowEvent::Resized(size) => {
                if let Some(on_window_resized) = callbacks.on_window_resized {
                    const U32_SIZE: usize = std::mem::size_of::<u32>();
                    let mut size: [u32; 2] = [size.width, size.height];
                    let ptr = size.as_mut_ptr().cast();
                    let size = crate::core::slice::nstd_core_slice_new(2, U32_SIZE, ptr);
                    on_window_resized(ncf, &mut window_id, &size);
                }
            }
            // A window has been repositioned.
            WindowEvent::Moved(pos) => {
                if let Some(on_window_moved) = callbacks.on_window_moved {
                    const I32_SIZE: usize = std::mem::size_of::<i32>();
                    let mut pos: [i32; 2] = [pos.x, pos.y];
                    let ptr = pos.as_mut_ptr().cast();
                    let pos = crate::core::slice::nstd_core_slice_new(2, I32_SIZE, ptr);
                    on_window_moved(ncf, &mut window_id, &pos);
                }
            }
            // A window's focus has changed.
            WindowEvent::Focused(is_focused) => {
                if let Some(on_window_focus_changed) = callbacks.on_window_focus_changed {
                    let is_focused = is_focused.into();
                    on_window_focus_changed(ncf, &mut window_id, is_focused);
                }
            }
            // A window has recieved keyboard input.
            WindowEvent::KeyboardInput {
                device_id, input, ..
            } => {
                let key = NSTDKeyEvent {
                    key: match input.virtual_keycode {
                        Some(key) => NSTDKey::from(key),
                        _ => NSTDKey::NSTD_KEY_NONE,
                    },
                    scan_code: input.scancode,
                    state: match input.state {
                        ElementState::Pressed => NSTDKeyState::NSTD_KEY_STATE_PRESSED,
                        ElementState::Released => NSTDKeyState::NSTD_KEY_STATE_RELEASED,
                    },
                };
                if let Some(on_window_keyboard_input) = callbacks.on_window_keyboard_input {
                    on_window_keyboard_input(ncf, &mut window_id, &device_id, &key);
                }
            }
            // A cursor was moved within a window's client area.
            WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => {
                if let Some(on_window_cursor_moved) = callbacks.on_window_cursor_moved {
                    const F64_SIZE: usize = std::mem::size_of::<f64>();
                    let mut pos: [f64; 2] = [position.x, position.y];
                    let ptr = pos.as_mut_ptr().cast();
                    let pos = crate::core::slice::nstd_core_slice_new(2, F64_SIZE, ptr);
                    on_window_cursor_moved(ncf, &mut window_id, &device_id, &pos);
                }
            }
            // The cursor entered a window.
            WindowEvent::CursorEntered { device_id } => {
                if let Some(on_window_cursor_entered) = callbacks.on_window_cursor_entered {
                    on_window_cursor_entered(ncf, &mut window_id, &device_id);
                }
            }
            // The cursor left a window.
            WindowEvent::CursorLeft { device_id } => {
                if let Some(on_window_cursor_left) = callbacks.on_window_cursor_left {
                    on_window_cursor_left(ncf, &mut window_id, &device_id);
                }
            }
            // A window is requesting to be closed.
            WindowEvent::CloseRequested => {
                if let Some(on_window_requests_closing) = callbacks.on_window_requests_closing {
                    on_window_requests_closing(ncf, &mut window_id);
                }
            }
            // A window has been closed/destroyed.
            WindowEvent::Destroyed => {
                if let Some(on_window_destroyed) = callbacks.on_window_destroyed {
                    on_window_destroyed(ncf, &mut window_id);
                }
            }
            _ => (),
        },
        _ => (),
    }
}

/// Frees an event loop without running it.
/// Parameters:
///     `NSTDEventLoop *const event_loop` - The event loop to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_events_event_loop_free(event_loop: *mut NSTDEventLoop) {
    Box::from_raw(*event_loop);
    *event_loop = std::ptr::null_mut();
}

/// Creates a new `NSTDEventCallbacks` with default callbacks.
/// Returns: `NSTDEventCallbacks callbacks` - The default event callbacks.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_events_event_callbacks_default() -> NSTDEventCallbacks {
    NSTDEventCallbacks {
        on_window_requests_closing: Some(on_window_requests_closing),
        ..Default::default()
    }
}

/// The default `on_window_requests_closing` implementation.
#[inline]
unsafe extern "C" fn on_window_requests_closing(
    control_flow: &mut NSTDEventLoopControlFlow,
    _: NSTDWindowID,
) {
    *control_flow = NSTD_EVENT_LOOP_CONTROL_FLOW_EXIT;
}
