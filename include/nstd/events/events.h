#ifndef NSTD_EVENTS_EVENTS_H_INCLUDED
#define NSTD_EVENTS_EVENTS_H_INCLUDED
#include "../core/def.h"
#include "../gui/def.h"
#include "../input.h"
#include "../nstd.h"
#include "def.h"
#include "device_id.h"
#include "window_id.h"
NSTDCPPSTART

/// An event loop handle.
typedef NSTDAny NSTDEventLoop;

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

/// Contains data about the event.
typedef struct
{
    /// The event loop's control flow.
    NSTDEventLoopControlFlow control_flow;
} NSTDEventData;

/// Event callbacks.
typedef struct
{
    /// Called when all main events have been processed.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    void (*on_update)(NSTDEventData *);
    /// Called when a device was added to the system registry.
    ///
    /// # Parameters:
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDDeviceID device_id` - The ID of the device.
    void (*on_device_added)(NSTDEventData *, NSTDDeviceID);
    /// Called when a device was removed from the system registry.
    ///
    /// # Parameters:
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDDeviceID device_id` - The ID of the device.
    void (*on_device_removed)(NSTDEventData *, NSTDDeviceID);
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
    void (*on_mouse_move)(NSTDEventData *, NSTDDeviceID, NSTDFloat64, NSTDFloat64);
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
    void (*on_mouse_scroll)(NSTDEventData *, NSTDDeviceID, NSTDFloat32, NSTDFloat32);
    /// Called when a button is pressed or released.
    ///
    /// # Parameters:
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDButtonID button_id` - The ID of the button.
    ///
    /// - `NSTDMouseButtonState state` - The state of the button.
    void (*on_button_input)(NSTDEventData *, NSTDButtonID, NSTDMouseButtonState);
    /// Called when a 'redraw requested' event is recieved.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    void (*on_redraw_requested)(NSTDEventData *, NSTDWindowID);
    /// Called after a window is resized.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `const NSTDWindowSize *size` - The new size of the window.
    void (*on_window_resized)(NSTDEventData *, NSTDWindowID, const NSTDWindowSize *);
    /// Called after a window is moved.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `const NSTDWindowPosition *size` - The new position of the window.
    void (*on_window_moved)(NSTDEventData *, NSTDWindowID, const NSTDWindowPosition *);
    /// Called when a window's focus changes.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `NSTDBool is_focused` - `NSTD_BOOL_TRUE` if the window gained focus.
    void (*on_window_focus_changed)(NSTDEventData *, NSTDWindowID, NSTDBool);
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
    void (*on_window_keyboard_input)(
        NSTDEventData *,
        NSTDWindowID,
        NSTDDeviceID,
        const NSTDKeyEvent *);
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
    void (*on_window_mouse_input)(
        NSTDEventData *,
        NSTDWindowID,
        NSTDDeviceID,
        const NSTDMouseButtonEvent *);
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
    void (*on_window_cursor_moved)(
        NSTDEventData *,
        NSTDWindowID,
        NSTDDeviceID,
        NSTDFloat64,
        NSTDFloat64);
    /// Called when a cursor enters a window.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `NSTDDeviceID device_id` - The device ID of the cursor.
    void (*on_window_cursor_entered)(NSTDEventData *, NSTDWindowID, NSTDDeviceID);
    /// Called when a cursor leaves a window.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    ///
    /// - `NSTDDeviceID device_id` - The device ID of the cursor.
    void (*on_window_cursor_left)(NSTDEventData *, NSTDWindowID, NSTDDeviceID);
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
    void (*on_window_line_scroll)(
        NSTDEventData *,
        NSTDWindowID,
        NSTDDeviceID,
        NSTDFloat32,
        NSTDFloat32);
    /// Called when a window requests closing.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window that requests closing.
    void (*on_window_requests_closing)(NSTDEventData *, NSTDWindowID);
    /// Called when a window is destroyed.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    ///
    /// - `NSTDWindowID window_id` - The ID of the window.
    void (*on_window_destroyed)(NSTDEventData *, NSTDWindowID);
    /// Called when the event loop is being destroyed.
    ///
    /// # Parameters
    ///
    /// - `NSTDEventData *event_data` - The control flow of the event loop.
    void (*on_destroy)(NSTDEventData *);
} NSTDEventCallbacks;

/// Creates a new event loop.
///
/// # Returns
///
/// `NSTDEventLoop event_loop` - The event loop.
NSTDAPI NSTDEventLoop nstd_events_event_loop_new();

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
NSTDAPI void nstd_events_event_loop_run(
    NSTDEventLoop *const event_loop,
    const NSTDEventCallbacks callbacks,
    const NSTDBool should_return);

/// Frees an event loop without running it.
///
/// # Parameters
///
/// - `NSTDEventLoop *const event_loop` - The event loop to free.
NSTDAPI void nstd_events_event_loop_free(NSTDEventLoop *const event_loop);

/// Creates a new `NSTDEventCallbacks` with default callbacks.
///
/// # Returns
///
/// `NSTDEventCallbacks callbacks` - The default event callbacks.
NSTDAPI NSTDEventCallbacks nstd_events_event_callbacks_default();

NSTDCPPEND
#endif
