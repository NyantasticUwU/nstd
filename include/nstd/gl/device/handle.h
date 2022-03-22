#ifndef NSTD_GL_DEVICE_HANDLE_H_INCLUDED
#define NSTD_GL_DEVICE_HANDLE_H_INCLUDED
#include "../../core/def.h"
#include "../../nstd.h"
#include "../def.h"
#include "../instance.h"
#include "../surface/surface.h"
#include "info.h"
NSTDCPPSTART

/// Represents a handle to a physical graphics device.
typedef NSTDAny NSTDGLDeviceHandle;

/// Gets a handle to a graphics device.
///
/// # Parameters
///
/// - `const NSTDGLInstance instance` - The `wgpu` instance.
///
/// - `const NSTDGLSurface surface` - The compatible surface.
///
/// - `const NSTDGLPowerPreference power_preference` - The amount of power the device should draw.
///
/// # Returns
///
/// `NSTDGLDeviceHandle device_handle` - A handle to a grahpics device.
NSTDAPI NSTDGLDeviceHandle nstd_gl_device_handle_new(
    const NSTDGLInstance instance,
    const NSTDGLSurface surface,
    const NSTDGLPowerPreference power_preference);

/// Retrieves info on a device.
///
/// # Parameters
///
/// - `const NSTDGLDeviceHandle device_handle` - Handle to a device.
///
/// # Returns
///
/// `NSTDGLDeviceInfo device_info` - Contains information about a device.
NSTDAPI NSTDGLDeviceInfo nstd_gl_device_handle_get_info(const NSTDGLDeviceHandle device_handle);

/// Frees a device handle.
///
/// # Parameters
///
/// - `NSTDGLDeviceHandle *const device_handle` - The device handle to free.
NSTDAPI void nstd_gl_device_handle_free(NSTDGLDeviceHandle *const device_handle);

NSTDCPPEND
#endif
