#ifndef NSTD_GL_SURFACE_TEXTURE_H_INCLUDED
#define NSTD_GL_SURFACE_TEXTURE_H_INCLUDED
#include "../../core/def.h"
#include "../../nstd.h"
#include "surface.h"
NSTDCPPSTART

/// A surface texture.
typedef NSTDAny NSTDGLSurfaceTexture;

/// Gets a surface's current texture.
///
/// # Parameters
///
/// - `const NSTDGLSurface surface` - The surface.
///
/// # Returns
///
/// `NSTDGLSurfaceTexture surface_texture` - The surface's texture.
NSTDAPI NSTDGLSurfaceTexture nstd_gl_surface_texture_current(const NSTDGLSurface surface);

/// Presents a surface texture to the surface.
///
/// # Note
///
/// This function will free `surface_texture`.
///
/// # Parameters
///
/// - `NSTDGLSurfaceTexture *const surface_texture` - The surface texture to present.
NSTDAPI void nstd_gl_surface_texture_present(NSTDGLSurfaceTexture *const surface_texture);

/// Frees a surface texture.
///
/// # Parameters
///
/// - `NSTDGLSurfaceTexture *const surface_texture` - The surface texture to free.
NSTDAPI void nstd_gl_surface_texture_free(NSTDGLSurfaceTexture *const surface_texture);

NSTDCPPEND
#endif
