//! Type used to identify a device.
use crate::core::def::NSTDBool;
use winit::event::DeviceId;

/// Represents a device ID.
pub type NSTDDeviceID = *mut DeviceId;

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_events_device_id_compare(
    id1: NSTDDeviceID,
    id2: NSTDDeviceID,
) -> NSTDBool {
    (*id1 == *id2).into()
}

/// Frees a device ID.
///
/// # Parameters
///
/// - `NSTDDeviceID *const device_id` - Pointer to the device ID.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_events_device_id_free(device_id: *mut NSTDDeviceID) {
    Box::from_raw(*device_id);
    *device_id = std::ptr::null_mut();
}
