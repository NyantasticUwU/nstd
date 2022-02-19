#ifndef NSTD_EVENTS_H_INCLUDED
#define NSTD_EVENTS_H_INCLUDED
#include "core/def.h"
#include "input.h"
#include "nstd.h"
#ifdef NSTDCPP
extern "C"
{
#endif

/// An event loop handle.
typedef NSTDAny NSTDEventLoop;

/// Represents a window ID.
typedef NSTDAny NSTDWindowID;

/// Represents an event loop's control flow.
typedef enum
{
    /// Event loop should poll after this iteration.
    NSTD_EVENT_LOOP_CONTROL_FLOW_POLL,
    /// Event loop should wait after this iteration.
    NSTD_EVENT_LOOP_CONTROL_FLOW_WAIT,
    /// Event loop should exit after this iteration.
    NSTD_EVENT_LOOP_CONTROL_FLOW_EXIT
} NSTDEventLoopControlFlow;

/// Represents an event.
typedef enum
{
    /// There is no event.
    NSTD_EVENT_LOOP_EVENT_NONE,
    /// The event loop is about to be destroyed.
    NSTD_EVENT_LOOP_EVENT_LOOP_DESTROYED,
    /// All events have been cleared.
    NSTD_EVENT_LOOP_EVENT_EVENTS_CLEARED,
    /// A device has been added.
    NSTD_EVENT_LOOP_EVENT_DEVICE_ADDED,
    /// A device has been removed.
    NSTD_EVENT_LOOP_EVENT_DEVICE_REMOVED,
    /// The mouse has been moved.
    NSTD_EVENT_LOOP_EVENT_MOUSE_MOVED,
    /// The scroll wheel was scrolled.
    NSTD_EVENT_LOOP_EVENT_SCROLL_PIXEL,
    /// The scroll wheel was scrolled.
    NSTD_EVENT_LOOP_EVENT_SCROLL_LINE,
    /// A window requests a redraw.
    NSTD_EVENT_LOOP_EVENT_WINDOW_REDRAW_REQUESTED,
    /// A window has been resized.
    NSTD_EVENT_LOOP_EVENT_WINDOW_RESIZED,
    /// A window was moved.
    NSTD_EVENT_LOOP_EVENT_WINDOW_MOVED,
    /// Window focus has changed.
    NSTD_EVENT_LOOP_EVENT_WINDOW_FOCUS_CHANGED,
    /// A keyboard key was pressed.
    NSTD_EVENT_LOOP_EVENT_WINDOW_KEY,
    /// A modifier key was pressed.
    NSTD_EVENT_LOOP_EVENT_WINDOW_MOD_KEY,
    /// The mouse has moved.
    NSTD_EVENT_LOOP_EVENT_WINDOW_MOUSE_MOVED,
    /// The mouse entered the window's frame.
    NSTD_EVENT_LOOP_EVENT_WINDOW_MOUSE_ENTERED,
    /// The mouse left the window's frame.
    NSTD_EVENT_LOOP_EVENT_WINDOW_MOUSE_LEFT,
    /// The scroll wheel was scrolled.
    NSTD_EVENT_LOOP_EVENT_WINDOW_SCROLL,
    /// A mouse button was clicked.
    NSTD_EVENT_LOOP_EVENT_WINDOW_MOUSE_BUTTON,
    /// A window requests closing.
    NSTD_EVENT_LOOP_EVENT_WINDOW_CLOSE_REQUESTED
} NSTDEventLoopEvent;

/// Holds an event's data.
typedef struct
{
    /// The event that was recieved.
    NSTDEventLoopEvent event;
    /// The difference in mouse position.
    NSTDFloat64 mouse_delta[2];
    /// A size.
    NSTDUInt32 size[2];
    /// A position.
    NSTDInt32 pos[2];
    /// The ID of a window.
    NSTDWindowID window_id;
    /// Raw input.
    NSTDRawInput raw_input;
    /// Touch state.
    NSTDTouchState touch_state;
    /// The mouse button event.
    NSTDMouseButtonEvent mouse_button_event;
    /// The key.
    NSTDKeyEvent key;
    /// The modifier keys.
    NSTDUInt8 mod_keys;
    /// `NSTD_BOOL_TRUE` if the window has focus.
    NSTDBool has_focus;
} NSTDEventData;

/// Creates a new event loop.
/// Returns: `NSTDEventLoop event_loop` - The event loop.
NSTDAPI NSTDEventLoop nstd_events_event_loop_new();

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
NSTDAPI void nstd_events_event_loop_run(
    NSTDEventLoop *const event_loop,
    NSTDEventLoopControlFlow(*callback)(NSTDEventData *),
    const NSTDBool should_return);

/// Frees an event loop without running it.
/// Parameters:
///     `NSTDEventLoop *const event_loop` - The event loop to free.
NSTDAPI void nstd_events_event_loop_free(NSTDEventLoop *const event_loop);

#ifdef NSTDCPP
}
#endif
#endif
