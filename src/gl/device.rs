use crate::{
    gl::{
        def::{NSTDGLBackend, NSTDGLPowerPreference},
        instance::NSTDGLInstance,
        surface::NSTDGLSurface,
    },
    string::NSTDString,
};
use wgpu::{Adapter, Device, DeviceType, Queue, RequestAdapterOptions};

/// Represents a handle to a physical graphics device.
pub type NSTDGLDeviceHandle = *mut Adapter;

/// Represents a graphics device.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDGLDevice {
    /// A raw pointer to the `wgpu` device.
    pub raw: *mut Device,
    /// The device's command queue.
    pub command_queue: *mut Queue,
}

/// Represents a device type.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash)]
pub enum NSTDGLDeviceType {
    /// An unknown device type.
    NSTD_GL_DEVICE_TYPE_UNKNOWN,
    /// `wgpu`'s integrated GPU.
    NSTD_GL_DEVICE_TYPE_INTEGRATED_GPU,
    /// A physical GPU.
    NSTD_GL_DEVICE_TYPE_DISCRETE_GPU,
    /// A virtual/hosted GPU.
    NSTD_GL_DEVICE_TYPE_VIRTUAL_GPU,
    /// CPU/Software rendering.
    NSTD_GL_DEVICE_TYPE_CPU,
}
impl Default for NSTDGLDeviceType {
    #[inline]
    fn default() -> Self {
        Self::NSTD_GL_DEVICE_TYPE_UNKNOWN
    }
}
impl From<DeviceType> for NSTDGLDeviceType {
    #[inline]
    fn from(device_type: DeviceType) -> Self {
        match device_type {
            DeviceType::Other => Self::NSTD_GL_DEVICE_TYPE_UNKNOWN,
            DeviceType::IntegratedGpu => Self::NSTD_GL_DEVICE_TYPE_INTEGRATED_GPU,
            DeviceType::DiscreteGpu => Self::NSTD_GL_DEVICE_TYPE_DISCRETE_GPU,
            DeviceType::VirtualGpu => Self::NSTD_GL_DEVICE_TYPE_VIRTUAL_GPU,
            DeviceType::Cpu => Self::NSTD_GL_DEVICE_TYPE_CPU,
        }
    }
}

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

/// Frees an `NSTDGLDeviceInfo` object.
/// Parameters:
///     `NSTDGLDeviceInfo *const device_info` - Pointer to an `NSTDGLDeviceInfo` object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_device_info_free(device_info: &mut NSTDGLDeviceInfo) {
    crate::string::nstd_string_free(&mut device_info.name);
}
