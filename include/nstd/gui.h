#ifndef NSTD_GUI_H_INCLUDED
#define NSTD_GUI_H_INCLUDED
#include "core/def.h"
#include "core/str.h"
#include "events.h"
#include "image.h"
#include "nstd.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a window.
typedef void *NSTDWindow;

/// Represents a display handle.
typedef void *NSTDDisplay;

/// Represents a window's position.
typedef struct
{
    NSTDInt32 x;
    NSTDInt32 y;
} NSTDWindowPosition;

/// Represents a window's size.
typedef struct
{
    NSTDUInt32 width;
    NSTDUInt32 height;
} NSTDWindowSize;

/// Creates a new window.
/// Parameters:
///     `NSTDEventLoop event_loop` - The event loop to attach to the window.
/// Returns: `NSTDWindow window` - The new window, null on error.
NSTDAPI NSTDWindow nstd_gui_window_create(NSTDEventLoop event_loop);

/// Creates a child window with `parent` being the parent window.
/// NOTE: This is only functional on Windows targets and will always return a null window handle on
/// any other platform.
/// Parameters:
///     `NSTDEventLoop event_loop` - The event loop to attach to the window.
///     `NSTDWindow parent` - The parent window.
/// Returns: `NSTDWindow child` - The new child window.
NSTDAPI NSTDWindow nstd_gui_window_create_child(NSTDEventLoop event_loop, NSTDWindow parent);

/// Requests the window to be drawn.
/// Parameters:
///     `NSTDWindow window` - The window.
NSTDAPI void nstd_gui_window_request_redraw(NSTDWindow window);

/// Gets a window's scale factor.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `NSTDFloat64 factor` - The scale factor of the window.
NSTDAPI NSTDFloat64 nstd_gui_window_get_scale_factor(NSTDWindow window);

/// Sets a window's position.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDWindowPosition pos` - The new position.
NSTDAPI void nstd_gui_window_set_position(NSTDWindow window, const NSTDWindowPosition pos);

/// Gets a window's position.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDWindowPosition *pos` - Returns as the position.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_gui_window_get_position(NSTDWindow window, NSTDWindowPosition *pos);

/// Gets a window's client position.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `NSTDWindowPosition *pos` - Returns as the position.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_gui_window_get_client_position(NSTDWindow window, NSTDWindowPosition *pos);

/// Gets a window's size.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `NSTDWindowSize size` - The size of the window.
NSTDAPI NSTDWindowSize nstd_gui_window_get_size(NSTDWindow window);

/// Sets a window's client size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDWindowSize size` - An array of 2 `NSTDUInt32`s.
NSTDAPI void nstd_gui_window_set_client_size(NSTDWindow window, const NSTDWindowSize size);

/// Gets a window's client size.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `NSTDWindowSize size` - The size of the window's client area.
NSTDAPI NSTDWindowSize nstd_gui_window_get_client_size(NSTDWindow window);

/// Sets a window's client min size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDWindowSize *const size` - An array of 2 `NSTDUInt32`s, null for no min.
NSTDAPI void nstd_gui_window_set_client_min_size(
    NSTDWindow window,
    const NSTDWindowSize *const size);

/// Sets a window's client max size.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDWindowSize *const size` - An array of 2 `NSTDUInt32`s, null for no max.
NSTDAPI void nstd_gui_window_set_client_max_size(
    NSTDWindow window,
    const NSTDWindowSize *const size);

/// Sets a window's title.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDStr *const title` - The new window title.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_gui_window_set_title(NSTDWindow window, const NSTDStr *const title);

/// Sets a window's visibility.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int visible` - Whether to show or hide the window.
NSTDAPI void nstd_gui_window_set_visible(NSTDWindow window, const int visible);

/// Sets whether the window is resizable or not.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int resizable` - Whether the window should be resizable or not.
NSTDAPI void nstd_gui_window_set_resizable(NSTDWindow window, const int resizable);

/// Sets the window's minimization mode.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int minimized` - Whether the window should be minimized or not.
NSTDAPI void nstd_gui_window_set_minimized(NSTDWindow window, const int minimized);

/// Sets the window's maximization mode.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int maximized` - Whether the window should be maximized or not.
NSTDAPI void nstd_gui_window_set_maximized(NSTDWindow window, const int maximized);

/// Checks if the window is maximized.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `int maximized` - Nonzero if the window is maximized.
NSTDAPI int nstd_gui_window_is_maximized(NSTDWindow window);

/// Sets a window's icon image.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const NSTDImage *const img` - The icon image, null for default.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_gui_window_set_icon(NSTDWindow window, const NSTDImage *const img);

/// Turn window decorations on or off.
/// Parameters:
///     `NSTDWindow window` - The window.
///     `const int decorations` - Whether to allow window decorations or not.
NSTDAPI void nstd_gui_window_set_decorations(NSTDWindow window, const int decorations);

/// Gets the window's ID.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `NSTDWindowID window_id` - The window ID.
NSTDAPI NSTDWindowID nstd_gui_window_get_id(NSTDWindow window);

/// Gets the display that the given window resides in.
/// Parameters:
///     `NSTDWindow window` - The window.
/// Returns: `NSTDDisplay display` - The display that the window is in.
NSTDAPI NSTDDisplay nstd_gui_window_get_display(NSTDWindow window);

/// Closes a window.
/// Parameters:
///     `NSTDWindow *window` - Pointer to the window.
NSTDAPI void nstd_gui_window_close(NSTDWindow *window);

/// Compares two window IDs.
/// Parameters:
///     `NSTDWindowID id1` - A window ID.
///     `NSTDWindowID id2` - Another window ID.
/// Returns: `int are_same` - 1 if the two IDs refer to the same window, 0 otherwise.
NSTDAPI int nstd_gui_window_id_compare(NSTDWindowID id1, NSTDWindowID id2);

/// Frees a window ID.
/// Parameters:
///     `NSTDWindowID *window_id` - Pointer to the window ID.
NSTDAPI void nstd_gui_window_id_free(NSTDWindowID *window_id);

/// Returns a display's size.
/// Parameters:
///     `NSTDDisplay display` - The display.
/// Returns: `NSTDWindowSize size` - The size of the display.
NSTDAPI NSTDWindowSize nstd_gui_display_get_size(NSTDDisplay display);

/// Returns the display's scale factor.
/// Parameters:
///     `NSTDDisplay display` - The display.
/// Returns: `NSTDFloat64 scale_factor` - The scale factor of the display.
NSTDAPI NSTDFloat64 nstd_gui_display_get_scale_factor(NSTDDisplay display);

/// Frees a display handle.
/// Parameters:
///     `NSTDDisplay *display` - Pointer to the display handle.
NSTDAPI void nstd_gui_display_free(NSTDDisplay *display);

#ifdef __cplusplus
}
#endif
#endif
