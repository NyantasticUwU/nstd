//! Types and functions for handling `nstd` events.
pub mod def;
pub mod device_id;
pub mod window_id;
use self::{
    def::{NSTDButtonID, NSTDWindowPosition, NSTDWindowSize},
    device_id::NSTDDeviceID,
    window_id::NSTDWindowID,
};
use crate::{
    core::def::NSTDBool,
    input::{
        key::{NSTDKey, NSTDKeyEvent, NSTDKeyState},
        mouse::{
            NSTDMouseButton::*,
            NSTDMouseButtonEvent,
            NSTDMouseButtonState::{self, *},
        },
    },
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
};

/// An event loop handle.
pub type NSTDEventLoop = *mut EventLoop<()>;

/// Represents an event loop's control flow.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash)]
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

/// Contains data about the event.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDEventData {
    /// The event loop's control flow.
    pub control_flow: NSTDEventLoopControlFlow,
}

/// Event callbacks.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NSTDEventCallbacks {
    /// Called when all main events have been processed.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    pub on_update: Option<unsafe extern "C" fn(&mut NSTDEventData)>,
    /// Called when a device was added to the system registry.
    ///
    /// # Parameters:
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDDeviceID device_id` - The ID of the device.
    pub on_device_added: Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDDeviceID)>,
    /// Called when a device was removed from the system registry.
    ///
    /// # Parameters:
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDDeviceID device_id` - The ID of the device.
    pub on_device_removed: Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDDeviceID)>,
    /// Called when keyboard input was received.
    ///
    /// # Parameters:
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDDeviceID device_id` - The ID of the keyboard.
    ///
    /// - `NSTDKeyEvent *key_event` - A pointer to the keyboard data.
    pub on_keyboard_input:
        Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDDeviceID, &NSTDKeyEvent)>,
    /// Called when a mouse cursor is moved.
    ///
    /// # Parameters:
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDDeviceID device_id` - The ID of the mouse.
    ///
    /// - `NSTDFloat64 x` - The number of pixels the cursor has moved on the x-axis.
    ///
    /// - `NSTDFloat64 y` - The number of pixels the cursor has moved on the y-axis.
    pub on_mouse_move: Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDDeviceID, f64, f64)>,
    /// Called when a mouse wheel is scrolled.
    ///
    /// # Parameters:
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDDeviceID device_id` - The ID of the mouse.
    ///
    /// - `NSTDFloat32 x` - The number of pixels the wheel has scrolled on the x-axis.
    ///
    /// - `NSTDFloat32 y` - The number of pixels the wheel has scrolled on the y-axis.
    pub on_mouse_scroll: Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDDeviceID, f32, f32)>,
    /// Called when a button is pressed or released.
    ///
    /// # Parameters:
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDButtonID button_id` - The ID of the button.
    ///
    /// - `NSTDMouseButtonState state` - The state of the button.
    pub on_button_input:
        Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDButtonID, NSTDMouseButtonState)>,
    /// Called when a 'redraw requested' event is recieved.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    pub on_redraw_requested: Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDWindowID)>,
    /// Called after a window is resized.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `const NSTDWindowSize *size` - The new size of the window..
    pub on_window_resized:
        Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDWindowID, &NSTDWindowSize)>,
    /// Called after a window is moved.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `const NSTDWindowPosition *size` - The new position of the window..
    pub on_window_moved:
        Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDWindowID, &NSTDWindowPosition)>,
    /// Called when a window's focus changes.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `NSTDBool is_focused` - `NSTD_BOOL_TRUE` if the window gained focus.
    pub on_window_focus_changed:
        Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDWindowID, NSTDBool)>,
    /// Called when a window recieve keyboard input.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `NSTDDeviceID device_id` - The device ID of the keyboard.
    ///
    /// - `const NSTDKeyEvent *key` - A pointer to the key data.
    pub on_window_keyboard_input:
        Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDWindowID, NSTDDeviceID, &NSTDKeyEvent)>,
    /// Called when a window recieves mouse input.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `NSTDDeviceID device_id` - The device ID of the mouse.
    ///
    /// - `const NSTDMouseButtonEvent *event` - The mouse event.
    pub on_window_mouse_input: Option<
        unsafe extern "C" fn(&mut NSTDEventData, NSTDWindowID, NSTDDeviceID, &NSTDMouseButtonEvent),
    >,
    /// Called when a cursor has moved within a window.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `NSTDDeviceID device_id` - The device ID of the cursor.
    ///
    /// - `NSTDFloat64 x` - The cursor's position on the x-axis.
    ///
    /// - `NSTDFloat64 y` - The cursor's position on the y-axis.
    pub on_window_cursor_moved:
        Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDWindowID, NSTDDeviceID, f64, f64)>,
    /// Called when a cursor enters a window.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `NSTDDeviceID device_id` - The device ID of the cursor.
    pub on_window_cursor_entered:
        Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDWindowID, NSTDDeviceID)>,
    /// Called when a cursor leaves a window.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `NSTDDeviceID device_id` - The device ID of the cursor.
    pub on_window_cursor_left:
        Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDWindowID, NSTDDeviceID)>,
    /// Called when a window is scrolled in units of lines or rows.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `NSTDDeviceID device_id` - The ID of the scroll wheel's device.
    ///
    /// - `NSTDFloat32 x` - The number of lines scrolled on the x-axis.
    ///
    /// - `NSTDFloat32 y` - The number of lines scrolled on the y-axis.
    pub on_window_line_scroll:
        Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDWindowID, NSTDDeviceID, f32, f32)>,
    /// Called when a window requests closing.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window that requests closing.
    pub on_window_requests_closing: Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDWindowID)>,
    /// Called when a window is destroyed.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    pub on_window_destroyed: Option<unsafe extern "C" fn(&mut NSTDEventData, NSTDWindowID)>,
    /// Called when the event loop is being destroyed.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    pub on_destroy: Option<unsafe extern "C" fn(&mut NSTDEventData)>,
}

/// Creates a new event loop.
///
/// # Returns
///
/// `NSTDEventLoop event_loop` - The event loop.
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
/// - Windows
/// - Linux
/// - MacOS
/// - Android
///
/// # Parameters
///
/// - `NSTDEventLoop *const event_loop` - The event loop to run.
///
/// - `const NSTDEventCallbacks callbacks` - The event callbacks.
///
/// - `const NSTDBool should_return` - `NSTD_BOOL_TRUE` if this function should return.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_events_event_loop_run(
    event_loop: &mut NSTDEventLoop,
    callbacks: NSTDEventCallbacks,
    should_return: NSTDBool,
) {
    // Regaining ownership of the winit event loop.
    let winit_event_loop = *event_loop;
    *event_loop = std::ptr::null_mut();
    let mut event_loop = Box::from_raw(winit_event_loop);
    // Creating the event handler closure.
    let closure =
        move |mut event: Event<()>, _: &EventLoopWindowTarget<()>, cf: &mut ControlFlow| {
            let mut event_data = NSTDEventData {
                control_flow: NSTDEventLoopControlFlow::NSTD_EVENT_LOOP_CONTROL_FLOW_POLL,
            };
            event_handler(&mut event, &mut event_data, &callbacks);
            *cf = event_data.control_flow.into();
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
    event: &mut Event<()>,
    ncf: &mut NSTDEventData,
    callbacks: &NSTDEventCallbacks,
) {
    match event {
        // All main events have been processed.
        Event::MainEventsCleared => {
            if let Some(on_update) = callbacks.on_update {
                on_update(ncf);
            }
        }
        // A window requests redrawing.
        Event::RedrawRequested(window_id) => {
            if let Some(on_redraw_requested) = callbacks.on_redraw_requested {
                on_redraw_requested(ncf, window_id);
            }
        }
        // The event loop is being destroyed.
        Event::LoopDestroyed => {
            if let Some(on_destroy) = callbacks.on_destroy {
                on_destroy(ncf);
            }
        }
        // A device event was recieved.
        Event::DeviceEvent { device_id, event } => match event {
            // A device was added.
            DeviceEvent::Added => {
                if let Some(on_device_added) = callbacks.on_device_added {
                    on_device_added(ncf, device_id);
                }
            }
            // A device was removed.
            DeviceEvent::Removed => {
                if let Some(on_device_removed) = callbacks.on_device_removed {
                    on_device_removed(ncf, device_id);
                }
            }
            // A key was pressed or released.
            DeviceEvent::Key(input) => {
                let key = NSTDKeyEvent {
                    key: match input.virtual_keycode {
                        Some(key) => NSTDKey::from(key),
                        _ => NSTDKey::default(),
                    },
                    scan_code: input.scancode,
                    state: match input.state {
                        ElementState::Pressed => NSTDKeyState::NSTD_KEY_STATE_PRESSED,
                        ElementState::Released => NSTDKeyState::NSTD_KEY_STATE_RELEASED,
                    },
                };
                if let Some(on_keyboard_input) = callbacks.on_keyboard_input {
                    on_keyboard_input(ncf, device_id, &key);
                }
            }
            // A mouse cursor was moved.
            DeviceEvent::MouseMotion { delta } => {
                if let Some(on_mouse_move) = callbacks.on_mouse_move {
                    on_mouse_move(ncf, device_id, delta.0, delta.1);
                }
            }
            // A mouse wheel was scrolled.
            DeviceEvent::MouseWheel { delta } => {
                if let MouseScrollDelta::LineDelta(x, y) = delta {
                    if let Some(on_mouse_scroll) = callbacks.on_mouse_scroll {
                        on_mouse_scroll(ncf, device_id, *x, *y);
                    }
                }
            }
            // A button's state was changed.
            DeviceEvent::Button { button, state } => {
                if let Some(on_button_input) = callbacks.on_button_input {
                    let state = match state {
                        ElementState::Pressed => NSTD_MOUSE_BUTTON_STATE_PRESSED,
                        ElementState::Released => NSTD_MOUSE_BUTTON_STATE_RELEASED,
                    };
                    on_button_input(ncf, *button, state);
                }
            }
            _ => (),
        },
        // A window event was recieved.
        Event::WindowEvent { event, window_id } => match event {
            // A window was resized.
            WindowEvent::Resized(size) => {
                if let Some(on_window_resized) = callbacks.on_window_resized {
                    let size = NSTDWindowSize::new(size.width, size.height);
                    on_window_resized(ncf, window_id, &size);
                }
            }
            // A window has been repositioned.
            WindowEvent::Moved(pos) => {
                if let Some(on_window_moved) = callbacks.on_window_moved {
                    let pos = NSTDWindowPosition::new(pos.x, pos.y);
                    on_window_moved(ncf, window_id, &pos);
                }
            }
            // A window's focus has changed.
            WindowEvent::Focused(is_focused) => {
                if let Some(on_window_focus_changed) = callbacks.on_window_focus_changed {
                    let is_focused = (*is_focused).into();
                    on_window_focus_changed(ncf, window_id, is_focused);
                }
            }
            // A window has recieved keyboard input.
            WindowEvent::KeyboardInput {
                device_id, input, ..
            } => {
                let key = NSTDKeyEvent {
                    key: match input.virtual_keycode {
                        Some(key) => NSTDKey::from(key),
                        _ => NSTDKey::default(),
                    },
                    scan_code: input.scancode,
                    state: match input.state {
                        ElementState::Pressed => NSTDKeyState::NSTD_KEY_STATE_PRESSED,
                        ElementState::Released => NSTDKeyState::NSTD_KEY_STATE_RELEASED,
                    },
                };
                if let Some(on_window_keyboard_input) = callbacks.on_window_keyboard_input {
                    on_window_keyboard_input(ncf, window_id, device_id, &key);
                }
            }
            // A window has recieved mouse input.
            WindowEvent::MouseInput {
                device_id,
                button,
                state,
                ..
            } => {
                if let Some(on_window_mouse_input) = callbacks.on_window_mouse_input {
                    let (button, extra_button) = match button {
                        MouseButton::Left => (NSTD_MOUSE_BUTTON_LEFT, u16::MAX),
                        MouseButton::Right => (NSTD_MOUSE_BUTTON_RIGHT, u16::MAX),
                        MouseButton::Middle => (NSTD_MOUSE_BUTTON_MIDDLE, u16::MAX),
                        MouseButton::Other(ibtn) => (NSTD_MOUSE_BUTTON_OTHER, *ibtn),
                    };
                    let event = NSTDMouseButtonEvent {
                        state: match state {
                            ElementState::Released => NSTD_MOUSE_BUTTON_STATE_RELEASED,
                            ElementState::Pressed => NSTD_MOUSE_BUTTON_STATE_PRESSED,
                        },
                        button,
                        extra_button,
                    };
                    on_window_mouse_input(ncf, window_id, device_id, &event);
                }
            }
            // A cursor was moved within a window's client area.
            WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => {
                if let Some(on_window_cursor_moved) = callbacks.on_window_cursor_moved {
                    on_window_cursor_moved(ncf, window_id, device_id, position.x, position.y);
                }
            }
            // The cursor entered a window.
            WindowEvent::CursorEntered { device_id } => {
                if let Some(on_window_cursor_entered) = callbacks.on_window_cursor_entered {
                    on_window_cursor_entered(ncf, window_id, device_id);
                }
            }
            // The cursor left a window.
            WindowEvent::CursorLeft { device_id } => {
                if let Some(on_window_cursor_left) = callbacks.on_window_cursor_left {
                    on_window_cursor_left(ncf, window_id, device_id);
                }
            }
            // A window was scrolled.
            WindowEvent::MouseWheel {
                device_id, delta, ..
            } => {
                if let MouseScrollDelta::LineDelta(x, y) = delta {
                    if let Some(on_window_line_scroll) = callbacks.on_window_line_scroll {
                        on_window_line_scroll(ncf, window_id, device_id, *x, *y);
                    }
                }
            }
            // A window is requesting to be closed.
            WindowEvent::CloseRequested => {
                if let Some(on_window_requests_closing) = callbacks.on_window_requests_closing {
                    on_window_requests_closing(ncf, window_id);
                }
            }
            // A window has been closed/destroyed.
            WindowEvent::Destroyed => {
                if let Some(on_window_destroyed) = callbacks.on_window_destroyed {
                    on_window_destroyed(ncf, window_id);
                }
            }
            _ => (),
        },
        _ => (),
    }
}

/// Frees an event loop without running it.
///
/// # Parameters
///
/// - `NSTDEventLoop *const event_loop` - The event loop to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_events_event_loop_free(event_loop: *mut NSTDEventLoop) {
    Box::from_raw(*event_loop);
    *event_loop = std::ptr::null_mut();
}

/// Creates a new `NSTDEventCallbacks` with default callbacks.
///
/// # Returns
///
/// `NSTDEventCallbacks callbacks` - The default event callbacks.
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
unsafe extern "C" fn on_window_requests_closing(event_data: &mut NSTDEventData, _: NSTDWindowID) {
    event_data.control_flow = NSTDEventLoopControlFlow::NSTD_EVENT_LOOP_CONTROL_FLOW_EXIT;
}
