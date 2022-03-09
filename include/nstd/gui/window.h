#ifndef NSTD_GUI_WINDOW_H_INCLUDED
#define NSTD_GUI_WINDOW_H_INCLUDED
#include "../core/def.h"
#include "../core/str.h"
#include "../events.h"
#include "../image.h"
#include "../nstd.h"
#include "def.h"
#include "display.h"
NSTDCPPSTART

/// Represents a window.
typedef NSTDAny NSTDWindow;

/// Creates a new window.
/// Parameters:
///     `const NSTDEventLoop event_loop` - The event loop to attach to the window.
/// Returns: `NSTDWindow window` - The new window, null on error.
NSTDAPI NSTDWindow nstd_gui_window_new(const NSTDEventLoop event_loop);

/// Creates a child window with `parent` being the parent window.
/// NOTE: This is only functional on Windows targets and will always return a null window handle on
/// any other platform.
/// Parameters:
///     `const NSTDEventLoop event_loop` - The event loop to attach to the window.
///     `const NSTDWindow parent` - The parent window.
/// Returns: `NSTDWindow child` - The new child window.
NSTDAPI NSTDWindow nstd_gui_window_new_child(
    const NSTDEventLoop event_loop,
    const NSTDWindow parent);

/// Requests the window to be drawn.
/// Parameters:
///     `const NSTDWindow window` - The window.
NSTDAPI void nstd_gui_window_request_redraw(const NSTDWindow window);

/// Gets a window's scale factor.
/// Parameters:
///     `const NSTDWindow window` - The window.
/// Returns: `NSTDFloat64 factor` - The scale factor of the window.
NSTDAPI NSTDFloat64 nstd_gui_window_get_scale_factor(const NSTDWindow window);

/// Sets a window's position.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDWindowPosition pos` - The new position.
NSTDAPI void nstd_gui_window_set_position(const NSTDWindow window, const NSTDWindowPosition pos);

/// Gets a window's position.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `NSTDWindowPosition *const pos` - Returns as the position.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_gui_window_get_position(
    const NSTDWindow window,
    NSTDWindowPosition *const pos);

/// Gets a window's client position.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `NSTDWindowPosition *const pos` - Returns as the position.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_gui_window_get_client_position(
    const NSTDWindow window,
    NSTDWindowPosition *const pos);

/// Gets a window's size.
/// Parameters:
///     `const NSTDWindow window` - The window.
/// Returns: `NSTDWindowSize size` - The size of the window.
NSTDAPI NSTDWindowSize nstd_gui_window_get_size(const NSTDWindow window);

/// Sets a window's client size.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDWindowSize size` - An array of 2 `NSTDUInt32`s.
NSTDAPI void nstd_gui_window_set_client_size(const NSTDWindow window, const NSTDWindowSize size);

/// Gets a window's client size.
/// Parameters:
///     `const NSTDWindow window` - The window.
/// Returns: `NSTDWindowSize size` - The size of the window's client area.
NSTDAPI NSTDWindowSize nstd_gui_window_get_client_size(const NSTDWindow window);

/// Sets a window's client min size.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDWindowSize *const size` - An array of 2 `NSTDUInt32`s, null for no min.
NSTDAPI void nstd_gui_window_set_client_min_size(
    const NSTDWindow window,
    const NSTDWindowSize *const size);

/// Sets a window's client max size.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDWindowSize *const size` - An array of 2 `NSTDUInt32`s, null for no max.
NSTDAPI void nstd_gui_window_set_client_max_size(
    const NSTDWindow window,
    const NSTDWindowSize *const size);

/// Sets a window's title.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDStr *const title` - The new window title.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_gui_window_set_title(
    const NSTDWindow window,
    const NSTDStr *const title);

/// Sets a window's visibility.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDBool visible` - Whether to show or hide the window.
NSTDAPI void nstd_gui_window_set_visible(const NSTDWindow window, const NSTDBool visible);

/// Sets whether the window is resizable or not.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDBool resizable` - Whether the window should be resizable or not.
NSTDAPI void nstd_gui_window_set_resizable(const NSTDWindow window, const NSTDBool resizable);

/// Sets the window's minimization mode.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDBool minimized` - Whether the window should be minimized or not.
NSTDAPI void nstd_gui_window_set_minimized(const NSTDWindow window, const NSTDBool minimized);

/// Sets the window's maximization mode.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDBool maximized` - Whether the window should be maximized or not.
NSTDAPI void nstd_gui_window_set_maximized(const NSTDWindow window, const NSTDBool maximized);

/// Checks if the window is maximized.
/// Parameters:
///     `const NSTDWindow window` - The window.
/// Returns: `NSTDBool maximized` - Nonzero if the window is maximized.
NSTDAPI NSTDBool nstd_gui_window_is_maximized(const NSTDWindow window);

/// Sets a window's icon image.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDImage *const img` - The icon image, null for default.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_gui_window_set_icon(const NSTDWindow window, const NSTDImage *const img);

/// Turn window decorations on or off.
/// Parameters:
///     `const NSTDWindow window` - The window.
///     `const NSTDBool decorations` - Whether to allow window decorations or not.
NSTDAPI void nstd_gui_window_set_decorations(const NSTDWindow window, const NSTDBool decorations);

/// Gets the window's ID.
/// Parameters:
///     `const NSTDWindow window` - The window.
/// Returns: `NSTDWindowID window_id` - The window ID.
NSTDAPI NSTDWindowID nstd_gui_window_get_id(const NSTDWindow window);

/// Gets the display that the given window resides in.
/// Parameters:
///     `const NSTDWindow window` - The window.
/// Returns: `NSTDDisplay display` - The display that the window is in.
NSTDAPI NSTDDisplay nstd_gui_window_get_display(const NSTDWindow window);

/// Closes a window.
/// Parameters:
///     `NSTDWindow *const window` - Pointer to the window.
NSTDAPI void nstd_gui_window_close(NSTDWindow *const window);

NSTDCPPEND
#endif
