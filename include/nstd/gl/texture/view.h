#ifndef NSTD_GL_TEXTURE_VIEW_H_INCLUDED
#define NSTD_GL_TEXTURE_VIEW_H_INCLUDED
#include "../../core/def.h"
#include "../../nstd.h"
#include "texture.h"
NSTDCPPSTART

/// Represents a view of a texture.
typedef NSTDAny NSTDGLTextureView;

/// Creates a new texture view.
/// Parameters:
///     `const NSTDGLSurfaceTexture surface_texture` - A surface texture.
/// Returns: `NSTDGLTextureView texture_view` - The new texture view.
NSTDAPI NSTDGLTextureView nstd_gl_texture_view_from_surface_texture(
    const NSTDGLSurfaceTexture surface_texture);

/// Frees a texture view.
/// Parameters:
///     `NSTDGLTextureView *const texture_view` - The texture view to free.
NSTDAPI void nstd_gl_texture_view_free(NSTDGLTextureView *const texture_view);

NSTDCPPEND
#endif
