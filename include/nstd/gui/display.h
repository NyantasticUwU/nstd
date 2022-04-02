#ifndef NSTD_GUI_DISPLAY_H_INCLUDED
#define NSTD_GUI_DISPLAY_H_INCLUDED
#include "../core/def.h"
#include "../events/def.h"
#include "../nstd.h"
NSTDCPPSTART

/// Represents a display handle.
typedef NSTDAny NSTDDisplay;

/// Returns a display's size.
///
/// # Parameters
///
/// - `const NSTDDisplay display` - The display.
///
/// # Returns
///
/// `NSTDWindowSize size` - The size of the display.
NSTDAPI NSTDWindowSize nstd_gui_display_get_size(const NSTDDisplay display);

/// Returns the display's scale factor.
///
/// # Parameters
///
/// - `const NSTDDisplay display` - The display.
///
/// # Returns
///
/// `NSTDFloat64 scale_factor` - The scale factor of the display.
NSTDAPI NSTDFloat64 nstd_gui_display_get_scale_factor(const NSTDDisplay display);

/// Frees a display handle.
///
/// # Parameters
///
/// - `NSTDDisplay *const display` - Pointer to the display handle.
NSTDAPI void nstd_gui_display_free(NSTDDisplay *const display);

NSTDCPPEND
#endif
