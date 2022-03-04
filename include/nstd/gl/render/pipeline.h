#ifndef NSTD_GL_RENDER_PIPELINE_H_INCLUDED
#define NSTD_GL_RENDER_PIPELINE_H_INCLUDED
#include "../../core/def.h"
#include "../../core/slice.h"
#include "../../nstd.h"
#include "../device/device.h"
#include "../shader/module.h"
#include "../surface/config.h"
NSTDCPPSTART

/// Represents a render pipeline.
typedef NSTDAny NSTDGLRenderPipeline;

/// Creates a new render pipeline from a vertex and fragment shader.
/// Parameters:
///     `const NSTDGLShaderModule vert` - The vertex shader module.
///     `const NSTDGLShaderModule frag` - The fragment shader module.
///     `const NSTDSlice *const buffers` - A slice of `NSTDGLVertexBufferLayout`s.
///     `const NSTDGLDevice device` - The device to create the render pipeline on.
///     `const NSTDGLSurfaceConfig config` - The surface configuration.
/// Returns: `NSTDGLRenderPipeline pipeline` - The new render pipeline.
NSTDAPI NSTDGLRenderPipeline nstd_gl_render_pipeline_new(
    const NSTDGLShaderModule vert,
    const NSTDGLShaderModule frag,
    const NSTDSlice *const buffers,
    const NSTDGLDevice device,
    const NSTDGLSurfaceConfig config);

/// Frees a render pipeline.
/// Parameters:
///     `NSTDGLRenderPipeline *const pipeline` - Pointer to a render pipeline.
NSTDAPI void nstd_gl_render_pipeline_free(NSTDGLRenderPipeline *const pipeline);

NSTDCPPEND
#endif
