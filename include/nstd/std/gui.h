#ifndef NSTD_STD_GUI_H_INCLUDED
#define NSTD_STD_GUI_H_INCLUDED
#include "../core/def.h"
#include "events.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a window.
typedef void *NSTDWindow;

/// Creates a new window.
/// Parameters:
///     `NSTDEventLoop event_loop` - The event loop to attach to the window.
/// Returns: `NSTDWindow window` - The new window, null on error.
NSTDAPI NSTDWindow nstd_std_gui_window_create(NSTDEventLoop event_loop);

/// Sets a window's title.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const char *const title` - The new window title.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_gui_window_set_title(NSTDWindow window, const char *const title);

#ifdef __cplusplus
}
#endif
#endif
