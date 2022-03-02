#ifndef NSTD_GL_SURFACE_H_INCLUDED
#define NSTD_GL_SURFACE_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "gui.h"
#include "instance.h"
NSTDCPPSTART

/// Represents a graphical surface.
typedef NSTDAny NSTDGLSurface;

/// Represents a surface config.
typedef NSTDAny NSTDGLSurfaceConfiguration;

/// Creates a new surface.
/// Parameters:
///     `const NSTDGLInstance instance` - The instance to create the surface with.
///     `const NSTDWindow window` - The window to get the surface from.
/// Returns: `NSTDGLSurface surface` - The surface of the window.
NSTDAPI NSTDGLSurface nstd_gl_surface_new(const NSTDGLInstance instance, const NSTDWindow window);

/// Frees a surface.
/// Parameters:
///     `NSTDGLSurface *const surface` - The surface to free.
NSTDAPI void nstd_gl_surface_free(NSTDGLSurface *const surface);

NSTDCPPEND
#endif
