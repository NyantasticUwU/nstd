#ifndef NSTD_GL_INSTANCE_H_INCLUDED
#define NSTD_GL_INSTANCE_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
NSTDCPPSTART

/// An instance of `nstd.gl`'s backend.
typedef NSTDAny NSTDGLInstance;

/// Creates a new instance of `nstd.gl`'s backend `wgpu`.
/// Returns: `NSTDGLInstance instance` - The `wgpu` instance.
NSTDAPI NSTDGLInstance nstd_gl_instance_new();

/// Frees an instance of `nstd.gl`'s backend.
/// Parameters:
///     `NSTDGLInstance *const instance` The instance to free.
NSTDAPI void nstd_gl_instance_free(NSTDGLInstance *const instance);

NSTDCPPEND
#endif
