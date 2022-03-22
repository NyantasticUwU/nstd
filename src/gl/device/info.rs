//! Information about a graphics device.
use crate::{
    gl::def::{NSTDGLBackend, NSTDGLDeviceType},
    string::NSTDString,
};

/// Contains information on a device.
#[repr(C)]
#[derive(Clone, Debug, Hash)]
pub struct NSTDGLDeviceInfo {
    /// The name of the drawing device.
    pub name: NSTDString,
    /// The device's vendor.
    pub vendor: usize,
    /// The ID of the device adapter.
    pub device: usize,
    /// The type of drawing device.
    pub device_type: NSTDGLDeviceType,
    /// The drawing backend in use.
    pub backend: NSTDGLBackend,
}

/// Frees an `NSTDGLDeviceInfo` object.
/// Parameters:
///     `NSTDGLDeviceInfo *const device_info` - Pointer to an `NSTDGLDeviceInfo` object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_device_info_free(device_info: &mut NSTDGLDeviceInfo) {
    crate::string::nstd_string_free(&mut device_info.name);
}
