use crate::{NSTDEvent::*, NSTDEventLoopControlFlow::*};
use nstd_input::key::{NSTDKey, NSTDKeyEvent, NSTDKeyState};
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
    event::{DeviceEvent, ElementState, Event, MouseScrollDelta, WindowEvent},
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
    NSTD_EVENT_WINDOW_CLOSE_REQUESTED,
}

/// Holds an event's data.
#[repr(C)]
#[derive(Default)]
pub struct NSTDEventData {
    mouse_delta: [c_double; 2],
    size: [u32; 2],
    pos: [i32; 2],
    key: NSTDKeyEvent,
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
) {
    let mut event_loop = Box::from_raw(event_loop as *mut EventLoop<()>);
    let mut event_data = NSTDEventData::default();
    let closure =
        move |event: Event<()>, _: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow| {
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
    event_loop.run_return(closure);
}
