use self::NSTDEvent::*;
use crate::input::{key::*, mouse::*, touch::NSTDTouchState, NSTDRawInput};
use std::{
    os::raw::{c_double, c_int},
    ptr::{self, addr_of_mut},
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
use winit_input_helper::WinitInputHelper;

/// An event loop handle.
pub type NSTDEventLoop = *mut EventLoop<()>;

/// Represents a window ID.
pub type NSTDWindowID = *mut WindowId;

/// Represents an event loop's control flow.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDEventLoopControlFlow {
    NSTD_EVENT_LOOP_CONTROL_FLOW_POLL,
    NSTD_EVENT_LOOP_CONTROL_FLOW_WAIT,
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
    NSTD_EVENT_NONE,
    NSTD_EVENT_LOOP_DESTROYED,
    NSTD_EVENT_EVENTS_CLEARED,
    NSTD_EVENT_DEVICE_ADDED,
    NSTD_EVENT_DEVICE_REMOVED,
    NSTD_EVENT_MOUSE_MOVED,
    NSTD_EVENT_SCROLL_PIXEL,
    NSTD_EVENT_SCROLL_LINE,
    NSTD_EVENT_WINDOW_REDRAW_REQUESTED,
    NSTD_EVENT_WINDOW_RESIZED,
    NSTD_EVENT_WINDOW_MOVED,
    NSTD_EVENT_WINDOW_FOCUS_CHANGED,
    NSTD_EVENT_WINDOW_KEY,
    NSTD_EVENT_WINDOW_MOD_KEY,
    NSTD_EVENT_WINDOW_MOUSE_MOVED,
    NSTD_EVENT_WINDOW_MOUSE_ENTERED,
    NSTD_EVENT_WINDOW_MOUSE_LEFT,
    NSTD_EVENT_WINDOW_SCROLL,
    NSTD_EVENT_WINDOW_MOUSE_BUTTON,
    NSTD_EVENT_WINDOW_CLOSE_REQUESTED,
}

/// Holds an event's data.
#[repr(C)]
pub struct NSTDEventData {
    pub event: NSTDEvent,
    pub mouse_delta: [c_double; 2],
    pub size: [u32; 2],
    pub pos: [i32; 2],
    pub window_id: NSTDWindowID,
    pub raw_input: NSTDRawInput,
    pub touch_state: NSTDTouchState,
    pub mouse_button_event: NSTDMouseButtonEvent,
    pub key: NSTDKeyEvent,
    pub mod_keys: u8,
    pub has_focus: i8,
}
impl Default for NSTDEventData {
    fn default() -> Self {
        Self {
            event: NSTD_EVENT_NONE,
            mouse_delta: [0.0, 0.0],
            size: [0, 0],
            pos: [0, 0],
            window_id: ptr::null_mut(),
            raw_input: ptr::null_mut(),
            touch_state: NSTDTouchState::default(),
            mouse_button_event: NSTDMouseButtonEvent::default(),
            key: NSTDKeyEvent::default(),
            mod_keys: 0,
            has_focus: 0,
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
///     `NSTDEventLoop *event_loop` - The event loop to run.
///     `NSTDEventLoopControlFlow(*callback)(NSTDEventData *)` - Called once per event.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_events_event_loop_run(
    event_loop: *mut NSTDEventLoop,
    callback: extern "C" fn(*mut NSTDEventData) -> NSTDEventLoopControlFlow,
    should_return: c_int,
) {
    let mut winit_event_loop = Box::from_raw(*event_loop);
    *event_loop = ptr::null_mut();
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
                            event_data.has_focus = focused as i8;
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
                                MouseScrollDelta::LineDelta(x, y) => [x as c_double, y as c_double],
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
                            event_data.mouse_delta = [x as c_double, y as c_double];
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
                event_data.window_id = ptr::null_mut();
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
    if should_return != 0 {
        winit_event_loop.run_return(closure);
    } else {
        winit_event_loop.run(closure);
    }
}

/// Frees an event loop without running it.
/// Parameters:
///     `NSTDEventLoop *event_loop` - The event loop to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_events_event_loop_free(event_loop: *mut NSTDEventLoop) {
    Box::from_raw(*event_loop);
    *event_loop = ptr::null_mut();
}
