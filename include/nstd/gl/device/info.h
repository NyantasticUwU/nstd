#ifndef NSTD_GL_DEVICE_INFO_H_INCLUDED
#define NSTD_GL_DEVICE_INFO_H_INCLUDED
#include "../../core/def.h"
#include "../../nstd.h"
#include "../../string.h"
#include "../def.h"
NSTDCPPSTART

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

/// Frees an `NSTDGLDeviceInfo` object.
/// Parameters:
///     `NSTDGLDeviceInfo *const device_info` - Pointer to an `NSTDGLDeviceInfo` object.
NSTDAPI void nstd_gl_device_info_free(NSTDGLDeviceInfo *const device_info);

NSTDCPPEND
#endif
