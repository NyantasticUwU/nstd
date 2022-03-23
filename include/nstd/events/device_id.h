#ifndef NSTD_EVENTS_DEVICE_ID_H_INCLUDED
#define NSTD_EVENTS_DEVICE_ID_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
NSTDCPPSTART

/// Represents a device ID.
typedef NSTDAny NSTDDeviceID;

/// Compares two device IDs.
///
/// # Parameters
///
/// - `const NSTDDeviceID id1` - A device ID.
///
/// - `const NSTDDeviceID id2` - Another device ID.
///
/// # Returns
///
/// `NSTDBool are_same` - `NSTD_BOOL_TRUE` if the two IDs refer to the same device.
NSTDAPI NSTDBool nstd_events_device_id_compare(const NSTDDeviceID id1, const NSTDDeviceID id2);

/// Frees a device ID.
///
/// # Parameters
///
/// - `NSTDDeviceID *const device_id` - Pointer to the device ID.
NSTDAPI void nstd_events_device_id_free(NSTDDeviceID *const device_id);

NSTDCPPEND
#endif
