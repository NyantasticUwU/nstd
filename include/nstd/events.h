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

/// Event callbacks.
typedef struct
{
    /// Called when all main events have been processed.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    void (*on_update)(NSTDEventLoopControlFlow *);
    /// Called when a window requests closing.
    /// Parameters:
    ///     `NSTDEventLoopControlFlow *control_flow` - The control flow of the event loop.
    ///     `NSTDWindowID window_id` - The ID of the window that requests closing.
    void (*on_window_requests_closing)(NSTDEventLoopControlFlow *, NSTDWindowID);
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
