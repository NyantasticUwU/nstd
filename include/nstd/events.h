#ifndef NSTD_EVENTS_H_INCLUDED
#define NSTD_EVENTS_H_INCLUDED
#include "core/def.h"
#include "core/slice.h"
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

/// Represents a device ID.
typedef NSTDAnyConst NSTDDeviceID;

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

/// Event callbacks.
typedef struct
{
    /// Called when all main events have been processed.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    void (*on_update)(NSTDEventLoopControlFlow *);
    /// Called when a 'redraw requested' event is recieved.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    void (*on_redraw_requested)(NSTDEventLoopControlFlow *, NSTDWindowID);
    /// Called after a window is resized.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `const NSTDSlice *size` - Two `NSTDUInt32`s representing 'width' and 'height'.
    void (*on_window_resized)(NSTDEventLoopControlFlow *, NSTDWindowID, const NSTDSlice *);
    /// Called after a window is moved.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `const NSTDSlice *size` - Two `NSTDInt32`s representing 'x' and 'y'.
    void (*on_window_moved)(NSTDEventLoopControlFlow *, NSTDWindowID, const NSTDSlice *);
    /// Called when a window's focus changes.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `NSTDBool is_focused` - `NSTD_BOOL_TRUE` if the window gained focus.
    void (*on_window_focus_changed)(NSTDEventLoopControlFlow *, NSTDWindowID, NSTDBool);
    /// Called when a window recieve's keyboard input.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `NSTDDeviceID device_id` - The device ID of the keyboard.
    ///     `const NSTDKeyEvent *key` - A pointer to the key data.
    void (*on_window_keyboard_input)(
        NSTDEventLoopControlFlow *,
        NSTDWindowID,
        NSTDDeviceID,
        const NSTDKeyEvent *);
    /// Called when a cursor has moved within a window.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `NSTDDeviceID device_id` - The device ID of the cursor.
    ///     `const NSTDSlice *pos` - Two `NSTDFloat64`s representing the cursor's position.
    void (*on_window_cursor_moved)(
        NSTDEventLoopControlFlow *,
        NSTDWindowID,
        NSTDDeviceID,
        const NSTDSlice *);
    /// Called when a cursor enters a window.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `NSTDDeviceID device_id` - The device ID of the cursor.
    void (*on_window_cursor_entered)(NSTDEventLoopControlFlow *, NSTDWindowID, NSTDDeviceID);
    /// Called when a cursor leaves a window.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    ///     `NSTDDeviceID device_id` - The device ID of the cursor.
    void (*on_window_cursor_left)(NSTDEventLoopControlFlow *, NSTDWindowID, NSTDDeviceID);
    /// Called when a window requests closing.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window that requests closing.
    void (*on_window_requests_closing)(NSTDEventLoopControlFlow *, NSTDWindowID);
    /// Called when a window is destroyed.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window.
    void (*on_window_destroyed)(NSTDEventLoopControlFlow *, NSTDWindowID);
    /// Called when the event loop is being destroyed.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    void (*on_destroy)(NSTDEventLoopControlFlow *);
} NSTDEventCallbacks;

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
///     `const NSTDEventCallbacks *const callbacks` - The event callbacks.
///     `const NSTDBool should_return` - `NSTD_BOOL_TRUE` if this function should return.
NSTDAPI void nstd_events_event_loop_run(
    NSTDEventLoop *const event_loop,
    const NSTDEventCallbacks *const callbacks,
    const NSTDBool should_return);

/// Frees an event loop without running it.
/// Parameters:
///     `NSTDEventLoop *const event_loop` - The event loop to free.
NSTDAPI void nstd_events_event_loop_free(NSTDEventLoop *const event_loop);

/// Creates a new `NSTDEventCallbacks` with default callbacks.
/// Returns: `NSTDEventCallbacks callbacks` - The default event callbacks.
NSTDAPI NSTDEventCallbacks nstd_events_event_callbacks_default();

#ifdef NSTDCPP
}
#endif
#endif
