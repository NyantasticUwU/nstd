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

/// Gets a window's scale factor.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `double factor` - The scale factor of the window.
NSTDAPI double nstd_std_gui_window_get_scale_factor(NSTDWindow window);

/// Sets a window's position.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDInt32 *const pos` - An array of 2 `NSTDInt32`s.
NSTDAPI void nstd_std_gui_window_set_position(NSTDWindow window, const NSTDInt32 *const pos);

/// Gets a window's position.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDInt32 *pos` - An array of 2 `NSTDInt32`s.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_gui_window_get_position(NSTDWindow window, NSTDInt32 *pos);

/// Gets a window's client position.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDInt32 *pos` - An array of 2 `NSTDInt32`s.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_gui_window_get_client_position(NSTDWindow window, NSTDInt32 *pos);

/// Gets a window's size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDUInt32 *size` - An array of 2 `NSTDUInt32`s.
NSTDAPI void nstd_std_gui_window_get_size(NSTDWindow window, NSTDUInt32 *size);

/// Sets a window's client size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDUInt32 *const size` - An array of 2 `NSTDUInt32`s.
NSTDAPI void nstd_std_gui_window_set_client_size(NSTDWindow window, const NSTDUInt32 *const size);

/// Gets a window's client size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDUInt32 *size` - An array of 2 `NSTDUInt32`s.
NSTDAPI void nstd_std_gui_window_get_client_size(NSTDWindow window, NSTDUInt32 *size);

/// Sets a window's client min size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDUInt32 *size` - An array of 2 `NSTDUInt32`s, null for no min.
NSTDAPI void nstd_std_gui_window_set_client_min_size(NSTDWindow window, NSTDUInt32 *size);

/// Sets a window's client max size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDUInt32 *size` - An array of 2 `NSTDUInt32`s, null for no max.
NSTDAPI void nstd_std_gui_window_set_client_max_size(NSTDWindow window, NSTDUInt32 *size);

/// Sets a window's title.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const char *const title` - The new window title.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_gui_window_set_title(NSTDWindow window, const char *const title);

/// Sets a window's visibility.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int visible` - Whether to show or hide the window.
NSTDAPI void nstd_std_gui_window_set_visible(NSTDWindow window, const int visible);

/// Sets whether the window is resizable or not.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int resizable` - Whether the window should be resizable or not.
NSTDAPI void nstd_std_gui_window_set_resizable(NSTDWindow window, const int resizable);

/// Sets the window's minimization mode.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int minimized` - Whether the window should be minimized or not.
NSTDAPI void nstd_std_gui_window_set_minimized(NSTDWindow window, const int minimized);

/// Sets the window's maximization mode.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int maximized` - Whether the window should be maximized or not.
NSTDAPI void nstd_std_gui_window_set_maximized(NSTDWindow window, const int maximized);

/// Checks if the window is maximized.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `int maximized` - Nonzero if the window is maximized.
NSTDAPI int nstd_std_gui_window_is_maximized(NSTDWindow window);

/// Turn window decorations on or off.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int decorations` - Whether to allow window decorations or not.
NSTDAPI void nstd_std_gui_window_set_decorations(NSTDWindow window, const int decorations);

#ifdef __cplusplus
}
#endif
#endif
