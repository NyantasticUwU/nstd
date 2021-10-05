use crate::{NSTDEvent::*, NSTDEventLoopControlFlow::*};
use nstd_input::{key::*, mouse::*, touch::NSTDTouchState};
use num_traits::FromPrimitive;
use std::{
    os::raw::{c_double, c_int, c_void},
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
};

/// An event loop handle.
pub type NSTDEventLoop = *mut c_void;

/// Represents an event loop's control flow.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDEventLoopControlFlow {
    NSTD_EVENT_LOOP_CONTROL_FLOW_POLL,
    NSTD_EVENT_LOOP_CONTROL_FLOW_WAIT,
    NSTD_EVENT_LOOP_CONTROL_FLOW_EXIT,
}

/// Represents an event.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDEvent {
    NSTD_EVENT_LOOP_DESTROYED,
    NSTD_EVENT_DEVICE_ADDED,
    NSTD_EVENT_DEVICE_REMOVED,
    NSTD_EVENT_MOUSE_MOVED,
    NSTD_EVENT_SCROLL_PIXEL,
    NSTD_EVENT_SCROLL_LINE,
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
#[derive(Default)]
pub struct NSTDEventData {
    mouse_delta: [c_double; 2],
    size: [u32; 2],
    pos: [i32; 2],
    touch_state: NSTDTouchState,
    mouse_button_event: NSTDMouseButtonEvent,
    key: NSTDKeyEvent,
    mod_keys: u8,
    has_focus: i8,
}

/// Creates a new event loop.
/// Returns: `NSTDEventLoop event_loop` - The event loop.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_events_event_loop_new() -> NSTDEventLoop {
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    return Box::into_raw(Box::new(EventLoop::new())) as NSTDEventLoop;
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    return Box::into_raw(Box::new(EventLoop::<()>::new_any_thread())) as NSTDEventLoop;
}

/// Runs an event loop, never returning.
/// Note that this function returns on the following operating systems:
///     - Windows
///     - Linux
///     - MacOS
///     - Android
/// Parameters:
///     `NSTDEventLoop event_loop` - The event loop to run.
///     `NSTDEventLoopControlFlow(*callback)(NSTDEvent *, NSTDEventData *)` - Called once per event.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_events_event_loop_run(
    event_loop: NSTDEventLoop,
    callback: extern "C" fn(*mut NSTDEvent, *mut NSTDEventData) -> NSTDEventLoopControlFlow,
    should_return: c_int,
) {
    let mut event_loop = Box::from_raw(event_loop as *mut EventLoop<()>);
    let mut event_data = NSTDEventData::default();
    let closure = move |event: Event<()>,
                        _: &EventLoopWindowTarget<()>,
                        control_flow: &mut ControlFlow| {
        let event = match event {
            Event::LoopDestroyed => Some(NSTD_EVENT_LOOP_DESTROYED),
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::Resized(size) => {
                    event_data.size = [size.width, size.height];
                    Some(NSTD_EVENT_WINDOW_RESIZED)
                }
                WindowEvent::Moved(pos) => {
                    event_data.pos = [pos.x, pos.y];
                    Some(NSTD_EVENT_WINDOW_MOVED)
                }
                WindowEvent::Focused(focused) => {
                    event_data.has_focus = focused as i8;
                    Some(NSTD_EVENT_WINDOW_FOCUS_CHANGED)
                }
                WindowEvent::KeyboardInput {
                    input,
                    device_id: _,
                    is_synthetic: _,
                } => {
                    event_data.key.state = match input.state {
                        ElementState::Pressed => NSTDKeyState::NSTD_KEY_STATE_PRESSED,
                        ElementState::Released => NSTDKeyState::NSTD_KEY_STATE_RELEASED,
                    };
                    event_data.key.scan_code = input.scancode;
                    event_data.key.key = match input.virtual_keycode {
                        // NSTDKey starts with `NONE` so add 1.
                        Some(key) => match FromPrimitive::from_i32((key as c_int) + 1) {
                            Some(key) => key,
                            _ => NSTDKey::NSTD_KEY_NONE,
                        },
                        _ => NSTDKey::NSTD_KEY_NONE,
                    };
                    Some(NSTD_EVENT_WINDOW_KEY)
                }
                WindowEvent::ModifiersChanged(mods) => {
                    event_data.mod_keys = 0
                        | NSTD_STD_INPUT_KEY_SHIFT_BIT * mods.shift() as u8
                        | NSTD_STD_INPUT_KEY_CTRL_BIT * mods.ctrl() as u8
                        | NSTD_STD_INPUT_KEY_ALT_BIT * mods.alt() as u8
                        | NSTD_STD_INPUT_KEY_LOGO_BIT * mods.logo() as u8;
                    Some(NSTD_EVENT_WINDOW_MOD_KEY)
                }
                #[allow(deprecated)]
                WindowEvent::CursorMoved {
                    position,
                    device_id: _,
                    modifiers: _,
                } => {
                    event_data.mouse_delta = [position.x, position.y];
                    Some(NSTD_EVENT_WINDOW_MOUSE_MOVED)
                }
                WindowEvent::CursorEntered { device_id: _ } => {
                    Some(NSTD_EVENT_WINDOW_MOUSE_ENTERED)
                }
                WindowEvent::CursorLeft { device_id: _ } => Some(NSTD_EVENT_WINDOW_MOUSE_LEFT),
                #[allow(deprecated)]
                WindowEvent::MouseWheel {
                    delta,
                    phase,
                    device_id: _,
                    modifiers: _,
                } => {
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
                    Some(NSTD_EVENT_WINDOW_SCROLL)
                }
                #[allow(deprecated)]
                WindowEvent::MouseInput {
                    state,
                    button,
                    device_id: _,
                    modifiers: _,
                } => {
                    event_data.mouse_button_event.state = match state {
                        ElementState::Pressed => NSTDMouseButtonState::NSTD_MOUSE_BUTTON_PRESSED,
                        ElementState::Released => NSTDMouseButtonState::NSTD_MOUSE_BUTTON_RELEASED,
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
                    Some(NSTD_EVENT_WINDOW_MOUSE_BUTTON)
                }
                WindowEvent::CloseRequested => Some(NSTD_EVENT_WINDOW_CLOSE_REQUESTED),
                _ => None,
            },
            Event::DeviceEvent {
                device_id: _,
                event,
            } => match event {
                DeviceEvent::Added => Some(NSTD_EVENT_DEVICE_ADDED),
                DeviceEvent::Removed => Some(NSTD_EVENT_DEVICE_REMOVED),
                DeviceEvent::MouseMotion { delta } => {
                    event_data.mouse_delta = [delta.0, delta.1];
                    Some(NSTD_EVENT_MOUSE_MOVED)
                }
                DeviceEvent::MouseWheel { delta } => match delta {
                    MouseScrollDelta::PixelDelta(delta) => {
                        event_data.mouse_delta = [delta.x, delta.y];
                        Some(NSTD_EVENT_SCROLL_PIXEL)
                    }
                    MouseScrollDelta::LineDelta(x, y) => {
                        event_data.mouse_delta = [x as c_double, y as c_double];
                        Some(NSTD_EVENT_SCROLL_LINE)
                    }
                },
                _ => None,
            },
            _ => None,
        };
        let cf = match event {
            Some(mut event) => callback(addr_of_mut!(event), addr_of_mut!(event_data)),
            None => callback(ptr::null_mut(), addr_of_mut!(event_data)),
        };
        *control_flow = match cf {
            NSTD_EVENT_LOOP_CONTROL_FLOW_POLL => ControlFlow::Poll,
            NSTD_EVENT_LOOP_CONTROL_FLOW_WAIT => ControlFlow::Wait,
            NSTD_EVENT_LOOP_CONTROL_FLOW_EXIT => ControlFlow::Exit,
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
        event_loop.run_return(closure);
    } else {
        event_loop.run(closure);
    }
}
