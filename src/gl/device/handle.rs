use crate::{
    gl::{
        def::{NSTDGLBackend, NSTDGLDeviceType, NSTDGLPowerPreference},
        device::info::NSTDGLDeviceInfo,
        instance::NSTDGLInstance,
        surface::NSTDGLSurface,
    },
    string::NSTDString,
};
use wgpu::{Adapter, RequestAdapterOptions};

/// Represents a handle to a physical graphics device.
pub type NSTDGLDeviceHandle = *mut Adapter;

/// Gets a handle to a graphics device.
/// Parameters:
///     `const NSTDGLInstance instance` - The `wgpu` instance.
///     `const NSTDGLSurface surface` - The compatible surface.
///     `const NSTDGLPowerPreference power_preference` - The amount of power the device should draw.
/// Returns: `NSTDGLDeviceHandle device_handle` - A handle to a grahpics device.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_device_handle_new(
    instace: NSTDGLInstance,
    surface: NSTDGLSurface,
    power_preference: NSTDGLPowerPreference,
) -> NSTDGLDeviceHandle {
    let adapter_options = RequestAdapterOptions {
        compatible_surface: Some(&*surface),
        power_preference: power_preference.into(),
        force_fallback_adapter: false,
    };
    match futures::executor::block_on((*instace).request_adapter(&adapter_options)) {
        Some(adapter) => Box::into_raw(Box::new(adapter)),
        _ => std::ptr::null_mut(),
    }
}

/// Retrieves info on a device.
/// Parameters:
///     `const NSTDGLDeviceHandle device_handle` - Handle to a device.
/// Returns: `NSTDGLDeviceInfo device_info` - Contains information about a device.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_device_handle_get_info(
    device_handle: NSTDGLDeviceHandle,
) -> NSTDGLDeviceInfo {
    let info = (*device_handle).get_info();
    NSTDGLDeviceInfo {
        name: NSTDString::from(info.name.as_bytes()),
        vendor: info.vendor,
        device: info.device,
        device_type: NSTDGLDeviceType::from(info.device_type),
        backend: NSTDGLBackend::from(info.backend),
    }
}

/// Frees a device handle.
/// Parameters:
///     `NSTDGLDeviceHandle *const device_handle` - The device handle to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_device_handle_free(device_handle: &mut NSTDGLDeviceHandle) {
    Box::from_raw(*device_handle);
    *device_handle = std::ptr::null_mut();
}
