#ifndef NSTD_GL_DEVICE_H_INCLUDED
#define NSTD_GL_DEVICE_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "../string.h"
#include "def.h"

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

#endif
