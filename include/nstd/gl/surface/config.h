#ifndef NSTD_GL_SURFACE_CONFIG_H_INCLUDED
#define NSTD_GL_SURFACE_CONFIG_H_INCLUDED
#include "../../core/def.h"
#include "../../gui/def.h"
#include "../../nstd.h"
#include "../def.h"
#include "../texture.h"
NSTDCPPSTART

/// Represents a surface config.
typedef NSTDAny NSTDGLSurfaceConfig;

/// Creates a new surface configuration.
/// Parameters:
///     `const NSTDWindowSize size` - The size of the viewport should be.
///     `const NSTDGLTextureFormat texture_format` - The texture format to use.
///     `const NSTDGLPresentationMode presentation_mode` - The presentation mode to use.
/// Returns: `NSTDGLSurfaceConfig config` - The surface configuration.
NSTDAPI NSTDGLSurfaceConfig nstd_gl_surface_config_new(
    const NSTDWindowSize size,
    const NSTDGLTextureFormat texture_format,
    const NSTDGLPresentationMode presentation_mode);

/// Frees a surface configuration.
/// Parameters:
///     `NSTDGLSurfaceConfiguration *const config` - The surface configuration to free.
NSTDAPI void nstd_gl_surface_config_free(NSTDGLSurfaceConfig *const config);

NSTDCPPEND
#endif
