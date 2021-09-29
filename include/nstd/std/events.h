#ifndef NSTD_STD_EVENTS_H_INCLUDED
#define NSTD_STD_EVENTS_H_INCLUDED
#include "../core/def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// An event loop handle.
typedef void *NSTDEventLoop;

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
    NSTD_EVENT_LOOP_DESTROYED,
    NSTD_EVENT_DEVICE_ADDED,
    NSTD_EVENT_DEVICE_REMOVED,
    NSTD_EVENT_MOUSE_MOVED,
    NSTD_EVENT_SCROLL_PIXEL,
    NSTD_EVENT_SCROLL_LINE,
    NSTD_EVENT_WINDOW_CLOSE_REQUESTED
} NSTDEvent;

/// Holds an event's data.
typedef struct
{
    double mouse_delta[2];
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
///     `NSTDEventLoop event_loop` - The event loop to run.
///     `NSTDEventLoopControlFlow(*callback)(NSTDEvent *, NSTDEventData *)` - Called once per event.
NSTDAPI void nstd_std_events_event_loop_run(
    NSTDEventLoop event_loop,
    NSTDEventLoopControlFlow(*callback)(NSTDEvent *, NSTDEventData *));

#ifdef __cplusplus
}
#endif
#endif
