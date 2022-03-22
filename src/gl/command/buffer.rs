//! GPU Command buffers.
use wgpu::CommandBuffer;

/// A command buffer for a graphics device.
pub type NSTDGLCommandBuffer = *mut CommandBuffer;

/// Frees a command buffer.
///
/// # Parameters
///
/// - `NSTDGLCommandBuffer *const command_buffer` - The gpu command buffer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_command_buffer_free(command_buffer: &mut NSTDGLCommandBuffer) {
    Box::from_raw(*command_buffer);
    *command_buffer = std::ptr::null_mut();
}
