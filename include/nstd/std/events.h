#ifndef NSTD_STD_EVENTS_H_INCLUDED
#define NSTD_STD_EVENTS_H_INCLUDED
#include "../core/def.h"
#include "def.h"
#include "input/input.h"
#include "input/key.h"
#include "input/mouse.h"
#include "input/touch.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// An event loop handle.
typedef void *NSTDEventLoop;

/// Represents a window ID.
typedef void *NSTDWindowID;

/// Represents an event loop's control flow.
typedef enum
{
    NSTD_EVENT_LOOP_CONTROL_FLOW_POLL,
    NSTD_EVENT_LOOP_CONTROL_FLOW_WAIT,
    NSTD_EVENT_LOOP_CONTROL_FLOW_EXIT
} NSTDEventLoopControlFlow;

/// Represents an event.
typedef enum
{
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
    NSTD_EVENT_WINDOW_CLOSE_REQUESTED
} NSTDEvent;

/// Holds an event's data.
typedef struct
{
    double mouse_delta[2];
    NSTDUInt32 size[2];
    NSTDInt32 pos[2];
    NSTDWindowID window_id;
    NSTDRawInput raw_input;
    NSTDTouchState touch_state;
    NSTDMouseButtonEvent mouse_button_event;
    NSTDKeyEvent key;
    NSTDUInt8 mod_keys;
    NSTDInt8 has_focus;
} NSTDEventData;

/// Creates a new event loop.
/// Returns: `NSTDEventLoop event_loop` - The event loop.
NSTDAPI NSTDEventLoop nstd_std_events_event_loop_new();

/// Runs an event loop, never returning.
/// Note that this function returns on the following operating systems:
///     - Windows
///     - Linux
///     - MacOS
///     - Android
/// Parameters:
///     `NSTDEventLoop *event_loop` - The event loop to run.
///     `NSTDEventLoopControlFlow(*callback)(NSTDEvent, NSTDEventData *)` - Called once per event.
NSTDAPI void nstd_std_events_event_loop_run(
    NSTDEventLoop *event_loop,
    NSTDEventLoopControlFlow(*callback)(NSTDEvent, NSTDEventData *),
    int should_return);

/// Frees an event loop without running it.
/// Parameters:
///     `NSTDEventLoop *event_loop` - The event loop to free.
NSTDAPI void nstd_std_events_event_loop_free(NSTDEventLoop *event_loop);

#ifdef __cplusplus
}
#endif
#endif
