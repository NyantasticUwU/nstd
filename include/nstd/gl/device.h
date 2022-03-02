#ifndef NSTD_GL_DEVICE_H_INCLUDED
#define NSTD_GL_DEVICE_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "../string.h"
#include "def.h"
NSTDCPPSTART

/// Represents a handle to a physical graphics device.
typedef NSTDAny NSTDGLDeviceHandle;

/// Represents a graphics device.
typedef NSTDAny NSTDGLDevice;

/// Represents a graphics device command queue.
typedef NSTDAny NSTDGLQueue;

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

/// Retrieves info on a device.
/// Parameters:
///     `const NSTDGLDeviceHandle device_handle` - Handle to a device.
/// Returns: `NSTDGLDeviceInfo device_info` - Contains information about a device.
NSTDAPI NSTDGLDeviceInfo nstd_gl_device_handle_get_info(const NSTDGLDeviceHandle device_handle);

/// Frees an `NSTDGLDeviceInfo` object.
/// Parameters:
///     `NSTDGLDeviceInfo *const device_info` - Pointer to an `NSTDGLDeviceInfo` object.
NSTDAPI void nstd_gl_device_info_free(NSTDGLDeviceInfo *const device_info);

NSTDCPPEND
#endif
