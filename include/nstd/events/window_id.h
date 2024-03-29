#ifndef NSTD_EVENTS_WINDOW_ID_H_INCLUDED
#define NSTD_EVENTS_WINDOW_ID_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
NSTDCPPSTART

/// Represents a window ID.
typedef NSTDAny NSTDWindowID;

/// Compares two window IDs.
///
/// # Parameters
///
/// - `const NSTDWindowID id1` - A window ID.
///
/// - `const NSTDWindowID id2` - Another window ID.
///
/// # Returns
///
/// `NSTDBool are_same` - `NSTD_BOOL_TRUE` if the two IDs refer to the same window.
NSTDAPI NSTDBool nstd_events_window_id_compare(const NSTDWindowID id1, const NSTDWindowID id2);

/// Frees a window ID.
///
/// # Parameters
///
/// - `NSTDWindowID *const window_id` - Pointer to the window ID.
NSTDAPI void nstd_events_window_id_free(NSTDWindowID *const window_id);

NSTDCPPEND
#endif
