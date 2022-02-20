use self::NSTDEventLoopControlFlow::*;
use crate::core::{def::NSTDBool, slice::NSTDSlice};
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
    ///     `NSTDWindowID window_id` - The ID of the window that requests closing.
    ///     `const NSTDSlice *size` - Two `NSTDUInt32`s representing 'width' and 'height'.
    pub on_window_resized:
        Option<unsafe extern "C" fn(&mut NSTDEventLoopControlFlow, NSTDWindowID, &NSTDSlice)>,
    /// Called after a window is moved.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window that requests closing.
    ///     `const NSTDSlice *size` - Two `NSTDInt32`s representing 'x' and 'y'.
    pub on_window_moved:
        Option<unsafe extern "C" fn(&mut NSTDEventLoopControlFlow, NSTDWindowID, &NSTDSlice)>,
    /// Called when a window requests closing.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window that requests closing.
    pub on_window_requests_closing:
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
        Event::MainEventsCleared => callbacks
            .on_update
            .and_then(|on_update| Some(on_update(ncf))),
        // The event loop is being destroyed.
        Event::LoopDestroyed => callbacks
            .on_destroy
            .and_then(|on_destroy| Some(on_destroy(ncf))),
        Event::WindowEvent {
            event,
            mut window_id,
        } => match event {
            // A window was resized.
            WindowEvent::Resized(size) => {
                callbacks.on_window_resized.and_then(|on_window_resized| {
                    const U32_SIZE: usize = std::mem::size_of::<u32>();
                    let mut size: [u32; 2] = [size.width, size.height];
                    let ptr = size.as_mut_ptr().cast();
                    let size = crate::core::slice::nstd_core_slice_new(2, U32_SIZE, ptr);
                    Some(on_window_resized(ncf, &mut window_id, &size))
                })
            }
            // A window has been repositioned.
            WindowEvent::Moved(pos) => callbacks.on_window_moved.and_then(|on_window_moved| {
                const I32_SIZE: usize = std::mem::size_of::<i32>();
                let mut pos: [i32; 2] = [pos.x, pos.y];
                let ptr = pos.as_mut_ptr().cast();
                let size = crate::core::slice::nstd_core_slice_new(2, I32_SIZE, ptr);
                Some(on_window_moved(ncf, &mut window_id, &size))
            }),
            // A window is requesting to be closed.
            WindowEvent::CloseRequested => match (*callbacks).on_window_requests_closing {
                Some(on_window_requests_closing) => {
                    Some(on_window_requests_closing(ncf, &mut window_id))
                }
                _ => None,
            },
            _ => None,
        },
        _ => None,
    };
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
