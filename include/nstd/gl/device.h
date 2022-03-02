#ifndef NSTD_GL_DEVICE_H_INCLUDED
#define NSTD_GL_DEVICE_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "../string.h"
#include "def.h"
#include "instance.h"
#include "surface.h"
NSTDCPPSTART

/// Represents a handle to a physical graphics device.
typedef NSTDAny NSTDGLDeviceHandle;

/// Represents a graphics device.
typedef struct
{
    /// A raw pointer to the `wgpu` device.
    NSTDAny raw;
    /// The device's command queue.
    NSTDAny command_queue;
} NSTDGLDevice;

/// Represents a device type.
typedef enum
{
    /// An unknown device type.
    NSTD_GL_DEVICE_TYPE_UNKNOWN,
    /// `wgpu`'s integrated GPU.
    NSTD_GL_DEVICE_TYPE_INTEGRATED_GPU,
    /// A physical GPU.
    NSTD_GL_DEVICE_TYPE_DISCRETE_GPU,
    /// A virtual/hosted GPU.
    NSTD_GL_DEVICE_TYPE_VIRTUAL_GPU,
    /// CPU/Software rendering.
    NSTD_GL_DEVICE_TYPE_CPU
} NSTDGLDeviceType;

/// Contains information on a device.
typedef struct
{
    /// The name of the drawing device.
    NSTDString name;
    /// The device's vendor.
    NSTDUSize vendor;
    /// The ID of the device adapter.
    NSTDUSize device;
    /// The type of drawing device.
    NSTDGLDeviceType device_type;
    /// The drawing backend in use.
    NSTDGLBackend backend;
} NSTDGLDeviceInfo;

/// Gets a handle to a graphics device.
/// Parameters:
///     `const NSTDGLInstance instance` - The `wgpu` instance.
///     `const NSTDGLSurface surface` - The compatible surface.
///     `const NSTDGLPowerPreference power_preference` - The amount of power the device should draw.
/// Returns: `NSTDGLDeviceHandle device_handle` - A handle to a grahpics device.
NSTDAPI NSTDGLDeviceHandle nstd_gl_device_handle_new(
    const NSTDGLInstance instance,
    const NSTDGLSurface surface,
    const NSTDGLPowerPreference power_preference);

/// Retrieves info on a device.
/// Parameters:
///     `const NSTDGLDeviceHandle device_handle` - Handle to a device.
/// Returns: `NSTDGLDeviceInfo device_info` - Contains information about a device.
NSTDAPI NSTDGLDeviceInfo nstd_gl_device_handle_get_info(const NSTDGLDeviceHandle device_handle);

/// Frees a device handle.
/// Parameters:
///     `NSTDGLDeviceHandle *const device_handle` - The device handle to free.
NSTDAPI void nstd_gl_device_handle_free(NSTDGLDeviceHandle *const device_handle);

/// Frees an `NSTDGLDeviceInfo` object.
/// Parameters:
///     `NSTDGLDeviceInfo *const device_info` - Pointer to an `NSTDGLDeviceInfo` object.
NSTDAPI void nstd_gl_device_info_free(NSTDGLDeviceInfo *const device_info);

NSTDCPPEND
#endif
