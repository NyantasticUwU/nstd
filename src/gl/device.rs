//! Module for working with graphics devices.
pub mod handle;
pub mod info;
use self::handle::NSTDGLDeviceHandle;
use crate::gl::command::buffer::NSTDGLCommandBuffer;
use wgpu::{Device, DeviceDescriptor, Queue};

/// Represents a graphics device.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDGLDevice {
    /// A raw pointer to the `wgpu` device.
    pub raw: *mut Device,
    /// The device's command queue.
    pub command_queue: *mut Queue,
}
impl Default for NSTDGLDevice {
    #[inline]
    fn default() -> Self {
        Self {
            raw: std::ptr::null_mut(),
            command_queue: std::ptr::null_mut(),
        }
    }
}

/// Creates a new device.
///
/// # Parameters
///
/// - `const NSTDGLDeviceHandle device_handle` - A handle to the device.
///
/// # Returns
///
/// `NSTDGLDevice device` - The physical device.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_device_new(device_handle: NSTDGLDeviceHandle) -> NSTDGLDevice {
    let request = (*device_handle).request_device(&DeviceDescriptor::default(), None);
    let (device, queue) = match futures::executor::block_on(request) {
        Ok((device, queue)) => (device, queue),
        _ => return NSTDGLDevice::default(),
    };
    NSTDGLDevice {
        raw: Box::into_raw(Box::new(device)),
        command_queue: Box::into_raw(Box::new(queue)),
    }
}

/// Submits a buffer of commands to a device.
///
/// # Parameters
///
/// - `const NSTDGLDevice device` - The device to submit commands to.
///
/// - `NSTDGLCommandBuffer *const command_buffer` - A device command buffer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_device_submit(
    device: NSTDGLDevice,
    command_buffer: &mut NSTDGLCommandBuffer,
) {
    let buffer = *Box::from_raw(*command_buffer);
    (*device.command_queue).submit(std::iter::once(buffer));
    *command_buffer = std::ptr::null_mut();
}

/// Frees a device.
///
/// # Parameters
///
/// - `NSTDGLDevice *const device` - A pointer to the device to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_device_free(device: &mut NSTDGLDevice) {
    Box::from_raw(device.raw);
    Box::from_raw(device.command_queue);
    device.raw = std::ptr::null_mut();
    device.command_queue = std::ptr::null_mut();
}
