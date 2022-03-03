#ifndef NSTD_GL_TEXTURE_H_INCLUDED
#define NSTD_GL_TEXTURE_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "device.h"
#include "surface/surface.h"
NSTDCPPSTART

/// A texture format.
typedef NSTDAny NSTDGLTextureFormat;

/// Gets the surface's preferred format.
/// Parameters:
///     `const NSTDGLSurface surface` - The surface.
///     `const NSTDGLDeviceHandle device_handle` - The device handle.
/// Returns: `NSTDGLTextureFormat texture_format` - The preferred texture format.
NSTDAPI NSTDGLTextureFormat nstd_gl_texture_format_default(
    const NSTDGLSurface surface,
    const NSTDGLDeviceHandle device_handle);

/// Frees an `NSTDGLTextureFormat`.
/// Parameters:
///     `NSTDGLTextureFormat *const texture_format` - The texture format to free.
NSTDAPI void nstd_gl_texture_format_free(NSTDGLTextureFormat *const texture_format);

NSTDCPPEND
#endif
