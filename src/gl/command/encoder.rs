//! GPU Command encoders.
use super::buffer::NSTDGLCommandBuffer;
use crate::gl::device::NSTDGLDevice;
use wgpu::{CommandEncoder, CommandEncoderDescriptor};

/// A command encoder for a graphics device.
pub type NSTDGLCommandEncoder = *mut CommandEncoder;

/// Creates a gpu command encoder.
///
/// # Parameters
///
/// - `const NSTDGLDevice *const device` - The device to create commands for.
///
/// # Returns
///
/// `NSTDGLCommandEncoder command_encoder` - The new gpu command encoder.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_command_encoder_new(
    device: &NSTDGLDevice,
) -> NSTDGLCommandEncoder {
    let encoder_desc = CommandEncoderDescriptor::default();
    let encoder = (*device.raw).create_command_encoder(&encoder_desc);
    Box::into_raw(Box::new(encoder))
}

/// Frees a command encoder and returns a command buffer.
///
/// # Parameters
///
/// - `NSTDGLCommandEncoder *const command_encoder` - The gpu command encoder.
///
/// # Returns
///
/// `NSTDGLCommandBuffer command_buffer` - The gpu command buffer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_command_encoder_finish(
    command_encoder: &mut NSTDGLCommandEncoder,
) -> NSTDGLCommandBuffer {
    // Free the command encoder.
    let encoder = Box::from_raw(*command_encoder);
    *command_encoder = std::ptr::null_mut();
    // Get the command buffer and return it.
    Box::into_raw(Box::new(encoder.finish()))
}

/// Frees a command encoder and submits the command buffer to the device immediately.
///
/// # Parameters
///
/// - `NSTDGLCommandEncoder *const command_encoder` - The gpu command encoder.
///
/// - `const NSTDGLDevice *const device` - The device to submit the command buffer to.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_command_encoder_finish_submit(
    command_encoder: &mut NSTDGLCommandEncoder,
    device: &NSTDGLDevice,
) {
    // Free the command encoder.
    let encoder = Box::from_raw(*command_encoder);
    *command_encoder = std::ptr::null_mut();
    // Get the command buffer and submit it.
    let buffer = encoder.finish();
    (*device.command_queue).submit(std::iter::once(buffer));
}
