#ifndef NSTD_GL_RENDER_PASS_H_INCLUDED
#define NSTD_GL_RENDER_PASS_H_INCLUDED
#include "../../core/def.h"
#include "../../nstd.h"
#include "../buffer.h"
#include "../command/encoder.h"
#include "../def.h"
#include "../texture/view.h"
#include "pipeline.h"
NSTDCPPSTART

/// Represents a render pass object.
typedef NSTDAny NSTDGLRenderPass;

/// Creates a new render pass.
///
/// # Parameters
///
/// - `const NSTDGLCommandEncoder command_encoder` - The command encoder.
///
/// - `const NSTDGLTextureView texture_view` - The texture view to render.
///
/// - `const NSTDGLColor *const clear_color` - The clear color.
///
/// # Returns
///
/// `NSTDGLRenderPass render_pass` - The new render pass.
NSTDAPI NSTDGLRenderPass nstd_gl_render_pass_new(
    const NSTDGLCommandEncoder command_encoder,
    const NSTDGLTextureView texture_view,
    const NSTDGLColor *const clear_color);

/// Sets a render pipeline for a render pass.
///
/// # Parameters
///
/// - `const NSTDGLRenderPass render_pass` - The render pass.
///
/// - `const NSTDGLRenderPipeline pipeline` - The render pipeline.
NSTDAPI void nstd_gl_render_pass_set_pipeline(
    const NSTDGLRenderPass render_pass,
    const NSTDGLRenderPipeline pipeline);

/// Sets a render pass' vertex buffer.
///
/// # Parameters
///
/// - `const NSTDGLRenderPass render_pass` - The render pass.
///
/// - `const NSTDGLBuffer buffer` - The GPU vertex buffer.
///
/// - `const NSTDUInt32 slot` - The buffer slot (the index of the buffer layout).
NSTDAPI void nstd_gl_render_pass_set_vertex_buffer(
    const NSTDGLRenderPass render_pass,
    const NSTDGLBuffer buffer,
    const NSTDUInt32 slot);

/// Sets a render pass' index buffer.
///
/// # Parameters
///
/// - `const NSTDGLRenderPass render_pass` - The render pass.
///
/// - `const NSTDGLBuffer buffer` - The GPU index buffer.
///
/// - `const NSTDGLIndexFormat format` - The index format of the buffer.
NSTDAPI void nstd_gl_render_pass_set_index_buffer(
    const NSTDGLRenderPass render_pass,
    const NSTDGLBuffer buffer,
    const NSTDGLIndexFormat format);

/// Draws primitives from active vertex buffers.
///
/// # Parameters
///
/// - `const NSTDGLRenderPass render_pass` - The render pass.
///
/// - `const NSTDUInt32 verticies` - Number of verticies to draw.
///
/// - `const NSTDUInt32 instances` - Number of instnaces.
NSTDAPI void nstd_gl_render_pass_draw(
    const NSTDGLRenderPass render_pass,
    const NSTDUInt32 verticies,
    const NSTDUInt32 instances);

/// Draws primitives from active vertex and index buffers.
///
/// # Parameters
///
/// - `const NSTDGLRenderPass render_pass` - The render pass.
///
/// - `const NSTDUInt32 indicies` - The indicies to draw.
///
/// - `const NSTDUInt32 instances` - The instances to draw.
///
/// - `const NSTDInt32 base` - The base vertex.
NSTDAPI void nstd_gl_render_pass_draw_indexed(
    const NSTDGLRenderPass render_pass,
    const NSTDUInt32 indicies,
    const NSTDUInt32 instances,
    const NSTDInt32 base);

/// Frees a render pass.
///
/// # Parameters
///
/// - `NSTDGLRenderPass *const render_pass` - The render pass to free.
NSTDAPI void nstd_gl_render_pass_free(NSTDGLRenderPass *const render_pass);

NSTDCPPEND
#endif
