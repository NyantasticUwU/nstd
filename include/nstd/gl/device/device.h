#ifndef NSTD_GL_DEVICE_DEVICE_H_INCLUDED
#define NSTD_GL_DEVICE_DEVICE_H_INCLUDED
#include "../../core/def.h"
#include "../../nstd.h"
#include "../command/buffer.h"
#include "handle.h"
NSTDCPPSTART

/// Represents a graphics device.
typedef struct
{
    /// A raw pointer to the `wgpu` device.
    NSTDAny raw;
    /// The device's command queue.
    NSTDAny command_queue;
} NSTDGLDevice;

/// Creates a new device.
///
/// # Parameters
///
/// - `const NSTDGLDeviceHandle device_handle` - A handle to the device.
///
/// # Returns
///
/// `NSTDGLDevice device` - The physical device.
NSTDAPI NSTDGLDevice nstd_gl_device_new(const NSTDGLDeviceHandle device_handle);

/// Submits a buffer of commands to a device.
///
/// # Parameters
///
/// - `const NSTDGLDevice device` - The device to submit commands to.
///
/// - `NSTDGLCommandBuffer *const command_buffer` - A device command buffer.
NSTDAPI void nstd_gl_device_submit(
    const NSTDGLDevice device,
    NSTDGLCommandBuffer *const command_buffer);

/// Frees a device.
///
/// # Parameters
///
/// - `NSTDGLDevice *const device` - A pointer to the device to free.
NSTDAPI void nstd_gl_device_free(NSTDGLDevice *const device);

NSTDCPPEND
#endif
