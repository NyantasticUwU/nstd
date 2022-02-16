use self::NSTDEvent::*;
use crate::{
    core::def::NSTDBool,
    input::{key::*, mouse::*, touch::NSTDTouchState, NSTDRawInput},
};
use std::ptr::addr_of_mut;
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
use winit_input_helper::WinitInputHelper;

/// An event loop handle.
pub type NSTDEventLoop = *mut EventLoop<()>;

/// Represents a window ID.
pub type NSTDWindowID = *mut WindowId;

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

/// Represents an event.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDEvent {
    /// There is no event.
    NSTD_EVENT_NONE,
    /// The event loop is about to be destroyed.
    NSTD_EVENT_LOOP_DESTROYED,
    /// All events have been cleared.
    NSTD_EVENT_EVENTS_CLEARED,
    /// A device has been added.
    NSTD_EVENT_DEVICE_ADDED,
    /// A device has been removed.
    NSTD_EVENT_DEVICE_REMOVED,
    /// The mouse has been moved.
    NSTD_EVENT_MOUSE_MOVED,
    /// The scroll wheel was scrolled.
    NSTD_EVENT_SCROLL_PIXEL,
    /// The scroll wheel was scrolled.
    NSTD_EVENT_SCROLL_LINE,
    /// A window requests a redraw.
    NSTD_EVENT_WINDOW_REDRAW_REQUESTED,
    /// A window has been resized.
    NSTD_EVENT_WINDOW_RESIZED,
    /// A window was moved.
    NSTD_EVENT_WINDOW_MOVED,
    /// Window focus has changed.
    NSTD_EVENT_WINDOW_FOCUS_CHANGED,
    /// A keyboard key was pressed.
    NSTD_EVENT_WINDOW_KEY,
    /// A modifier key was pressed.
    NSTD_EVENT_WINDOW_MOD_KEY,
    /// The mouse has moved.
    NSTD_EVENT_WINDOW_MOUSE_MOVED,
    /// The mouse entered the window's frame.
    NSTD_EVENT_WINDOW_MOUSE_ENTERED,
    /// The mouse left the window's frame.
    NSTD_EVENT_WINDOW_MOUSE_LEFT,
    /// The scroll wheel was scrolled.
    NSTD_EVENT_WINDOW_SCROLL,
    /// A mouse button was clicked.
    NSTD_EVENT_WINDOW_MOUSE_BUTTON,
    /// A window requests closing.
    NSTD_EVENT_WINDOW_CLOSE_REQUESTED,
}

/// Holds an event's data.
#[repr(C)]
pub struct NSTDEventData {
    /// The event that was recieved.
    pub event: NSTDEvent,
    /// The difference in mouse position.
    pub mouse_delta: [f64; 2],
    /// A size.
    pub size: [u32; 2],
    /// A position.
    pub pos: [i32; 2],
    /// The ID of a window.
    pub window_id: NSTDWindowID,
    /// Raw input.
    pub raw_input: NSTDRawInput,
    /// Touch state.
    pub touch_state: NSTDTouchState,
    /// The mouse button event.
    pub mouse_button_event: NSTDMouseButtonEvent,
    /// The key.
    pub key: NSTDKeyEvent,
    /// The modifier keys.
    pub mod_keys: u8,
    /// Nonzero if the window has focus.
    pub has_focus: NSTDBool,
}
impl Default for NSTDEventData {
    fn default() -> Self {
        Self {
            event: NSTD_EVENT_NONE,
            mouse_delta: [0.0, 0.0],
            size: [0, 0],
            pos: [0, 0],
            window_id: std::ptr::null_mut(),
            raw_input: std::ptr::null_mut(),
            touch_state: NSTDTouchState::default(),
            mouse_button_event: NSTDMouseButtonEvent::default(),
            key: NSTDKeyEvent::default(),
            mod_keys: 0,
            has_focus: NSTDBool::NSTD_BOOL_FALSE,
        }
    }
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
///     `NSTDEventLoopControlFlow(*callback)(NSTDEventData *)` - Called once per event.
///     `const NSTDBool should_return` - `NSTD_BOOL_TRUE` if this function should return.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_events_event_loop_run(
    event_loop: *mut NSTDEventLoop,
    callback: extern "C" fn(*mut NSTDEventData) -> NSTDEventLoopControlFlow,
    should_return: NSTDBool,
) {
    let mut winit_event_loop = Box::from_raw(*event_loop);
    *event_loop = std::ptr::null_mut();
    let mut winput = Box::new(WinitInputHelper::new());
    let mut event_data = NSTDEventData::default();
    event_data.raw_input = winput.as_mut();
    let closure =
        move |event: Event<()>, _: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow| {
            winput.update(&event);
            event_data.event = match event {
                Event::LoopDestroyed => NSTD_EVENT_LOOP_DESTROYED,
                Event::MainEventsCleared => NSTD_EVENT_EVENTS_CLEARED,
                Event::RedrawRequested(window_id) => {
                    event_data.window_id = Box::into_raw(Box::new(window_id));
                    NSTD_EVENT_WINDOW_REDRAW_REQUESTED
                }
                Event::WindowEvent { window_id, event } => {
                    event_data.window_id = Box::into_raw(Box::new(window_id));
                    match event {
                        WindowEvent::Resized(size) => {
                            event_data.size = [size.width, size.height];
                            NSTD_EVENT_WINDOW_RESIZED
                        }
                        WindowEvent::Moved(pos) => {
                            event_data.pos = [pos.x, pos.y];
                            NSTD_EVENT_WINDOW_MOVED
                        }
                        WindowEvent::Focused(focused) => {
                            event_data.has_focus = NSTDBool::from(focused);
                            NSTD_EVENT_WINDOW_FOCUS_CHANGED
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            event_data.key.state = match input.state {
                                ElementState::Pressed => NSTDKeyState::NSTD_KEY_STATE_PRESSED,
                                ElementState::Released => NSTDKeyState::NSTD_KEY_STATE_RELEASED,
                            };
                            event_data.key.scan_code = input.scancode;
                            event_data.key.key = match input.virtual_keycode {
                                Some(key) => NSTDKey::from(key),
                                _ => NSTDKey::NSTD_KEY_NONE,
                            };
                            NSTD_EVENT_WINDOW_KEY
                        }
                        WindowEvent::ModifiersChanged(mods) => {
                            event_data.mod_keys = 0
                                | NSTD_INPUT_KEY_SHIFT_BIT * mods.shift() as u8
                                | NSTD_INPUT_KEY_CTRL_BIT * mods.ctrl() as u8
                                | NSTD_INPUT_KEY_ALT_BIT * mods.alt() as u8
                                | NSTD_INPUT_KEY_LOGO_BIT * mods.logo() as u8;
                            NSTD_EVENT_WINDOW_MOD_KEY
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            event_data.mouse_delta = [position.x, position.y];
                            NSTD_EVENT_WINDOW_MOUSE_MOVED
                        }
                        WindowEvent::CursorEntered { .. } => NSTD_EVENT_WINDOW_MOUSE_ENTERED,
                        WindowEvent::CursorLeft { .. } => NSTD_EVENT_WINDOW_MOUSE_LEFT,
                        WindowEvent::MouseWheel { delta, phase, .. } => {
                            event_data.mouse_delta = match delta {
                                MouseScrollDelta::PixelDelta(delta) => [delta.x, delta.y],
                                MouseScrollDelta::LineDelta(x, y) => [x as f64, y as f64],
                            };
                            event_data.touch_state = match phase {
                                TouchPhase::Started => NSTDTouchState::NSTD_TOUCH_STATE_STARTED,
                                TouchPhase::Moved => NSTDTouchState::NSTD_TOUCH_STATE_MOVED,
                                TouchPhase::Ended => NSTDTouchState::NSTD_TOUCH_STATE_ENDED,
                                TouchPhase::Cancelled => NSTDTouchState::NSTD_TOUCH_STATE_CANCELLED,
                            };
                            NSTD_EVENT_WINDOW_SCROLL
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            event_data.mouse_button_event.state = match state {
                                ElementState::Pressed => {
                                    NSTDMouseButtonState::NSTD_MOUSE_BUTTON_PRESSED
                                }
                                ElementState::Released => {
                                    NSTDMouseButtonState::NSTD_MOUSE_BUTTON_RELEASED
                                }
                            };
                            event_data.mouse_button_event.button = match button {
                                MouseButton::Left => NSTDMouseButton::NSTD_MOUSE_BUTTON_LEFT,
                                MouseButton::Right => NSTDMouseButton::NSTD_MOUSE_BUTTON_RIGHT,
                                MouseButton::Middle => NSTDMouseButton::NSTD_MOUSE_BUTTON_MIDDLE,
                                MouseButton::Other(other) => {
                                    event_data.mouse_button_event.extra_button = other;
                                    NSTDMouseButton::NSTD_MOUSE_BUTTON_OTHER
                                }
                            };
                            NSTD_EVENT_WINDOW_MOUSE_BUTTON
                        }
                        WindowEvent::CloseRequested => NSTD_EVENT_WINDOW_CLOSE_REQUESTED,
                        _ => NSTD_EVENT_NONE,
                    }
                }
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::Added => NSTD_EVENT_DEVICE_ADDED,
                    DeviceEvent::Removed => NSTD_EVENT_DEVICE_REMOVED,
                    DeviceEvent::MouseMotion { delta } => {
                        event_data.mouse_delta = [delta.0, delta.1];
                        NSTD_EVENT_MOUSE_MOVED
                    }
                    DeviceEvent::MouseWheel { delta } => match delta {
                        MouseScrollDelta::PixelDelta(delta) => {
                            event_data.mouse_delta = [delta.x, delta.y];
                            NSTD_EVENT_SCROLL_PIXEL
                        }
                        MouseScrollDelta::LineDelta(x, y) => {
                            event_data.mouse_delta = [x as f64, y as f64];
                            NSTD_EVENT_SCROLL_LINE
                        }
                    },
                    _ => NSTD_EVENT_NONE,
                },
                _ => NSTD_EVENT_NONE,
            };
            *control_flow = callback(addr_of_mut!(event_data)).into();
            if !event_data.window_id.is_null() {
                Box::from_raw(event_data.window_id);
                event_data.window_id = std::ptr::null_mut();
            }
        };
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
    if should_return != NSTDBool::NSTD_BOOL_FALSE {
        winit_event_loop.run_return(closure);
    } else {
        winit_event_loop.run(closure);
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
