#ifndef NSTD_GL_GL_H_INCLUDED
#define NSTD_GL_GL_H_INCLUDED
#include "../core/def.h"
#include "../core/slice.h"
#include "../gui.h"
#include "../nstd.h"
#include "../string.h"
#include "buffer.h"
#include "def.h"
#include "device.h"
#include "pipeline.h"
#include "state.h"
#include "surface.h"
NSTDCPPSTART

/// Retrieves info on a device.
/// Parameters:
///     `const NSTDGLDeviceHandle device_handle` - Handle to a device.
/// Returns: `NSTDGLDeviceInfo device_info` - Contains information about a device.
NSTDAPI NSTDGLDeviceInfo nstd_gl_device_handle_get_info(const NSTDGLDeviceHandle device_handle);

/// Creates a new shader module.
/// Parameters:
///     `const NSTDSlice *const data` - Raw spirv data.
///     `const NSTDGLDevice device` - The device to create the shader module on.
/// Returns: `NSTDGLShaderModule shader` - The new shader module.
NSTDAPI NSTDGLShaderModule nstd_gl_shader_module_new(
    const NSTDSlice *const data,
    const NSTDGLDevice device);

/// Frees a shader module.
/// Parameters:
///     `NSTDGLShaderModule *const shader` - Pointer to a shader module.
NSTDAPI void nstd_gl_shader_module_free(NSTDGLShaderModule *const shader);

/// Creates a new render pipeline from a vertex and fragment shader.
/// Parameters:
///     `const NSTDGLShaderModule vert` - The vertex shader module.
///     `const NSTDGLShaderModule frag` - The fragment shader module.
///     `const NSTDSlice *const buffers` - A slice of `NSTDGLVertexBufferLayout`s.
///     `const NSTDGLDevice device` - The device to create the render pipeline on.
///     `const NSTDGLSurfaceConfiguration config` - The surface configuration.
/// Returns: `NSTDGLRenderPipeline pipeline` - The new render pipeline.
NSTDAPI NSTDGLRenderPipeline nstd_gl_render_pipeline_new(
    const NSTDGLShaderModule vert,
    const NSTDGLShaderModule frag,
    const NSTDSlice *const buffers,
    const NSTDGLDevice device,
    const NSTDGLSurfaceConfiguration config);

/// Frees a render pipeline.
/// Parameters:
///     `NSTDGLRenderPipeline *const pipeline` - Pointer to a render pipeline.
NSTDAPI void nstd_gl_render_pipeline_free(NSTDGLRenderPipeline *const pipeline);

/// Sets a render pipeline for a render pass.
/// Parameters:
///     `const NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDGLRenderPipeline pipeline` - The render pipeline.
NSTDAPI void nstd_gl_render_pass_set_pipeline(
    const NSTDGLRenderPass render_pass,
    const NSTDGLRenderPipeline pipeline);

/// Sets a render pass' vertex buffer.
/// Parameters:
///     `const NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDGLBuffer buffer` - The GPU vertex buffer.
///     `const NSTDUInt32 slot` - The buffer slot (the index of the buffer layout).
NSTDAPI void nstd_gl_render_pass_set_vertex_buffer(
    const NSTDGLRenderPass render_pass,
    const NSTDGLBuffer buffer,
    const NSTDUInt32 slot);

/// Sets a render pass' index buffer.
/// Parameters:
///     `const NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDGLBuffer buffer` - The GPU index buffer.
///     `const NSTDGLIndexFormat format` - The index format of the buffer.
NSTDAPI void nstd_gl_render_pass_set_index_buffer(
    const NSTDGLRenderPass render_pass,
    const NSTDGLBuffer buffer,
    const NSTDGLIndexFormat format);

/// Draws primitives from active vertex buffers.
/// Parameters:
///     `const NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDUInt32 verticies` - Number of verticies to draw.
///     `const NSTDUInt32 instances` - Number of instnaces.
NSTDAPI void nstd_gl_render_pass_draw(
    const NSTDGLRenderPass render_pass,
    const NSTDUInt32 verticies,
    const NSTDUInt32 instances);

/// Draws primitives from active vertex and index buffers.
/// Parameters:
///     `const NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDUInt32 indicies` - The indicies to draw.
///     `const NSTDUInt32 instances` - The instances to draw.
///     `const NSTDInt32 base` - The base vertex.
NSTDAPI void nstd_gl_render_pass_draw_indexed(
    const NSTDGLRenderPass render_pass,
    const NSTDUInt32 indicies,
    const NSTDUInt32 instances,
    const NSTDInt32 base);

/// Frees an `NSTDGLDeviceInfo` object.
/// Parameters:
///     `NSTDGLDeviceInfo *const device_info` - Pointer to an `NSTDGLDeviceInfo` object.
NSTDAPI void nstd_gl_device_info_free(NSTDGLDeviceInfo *const device_info);

/// Creates a new GPU buffer.
/// Parameters:
///     `const NSTDSlice *const bytes` - The bytes to send to the GPU.
///     `const NSTDGLDevice device` - The device to create the buffer on.
/// Returns: `NSTDGLBuffer buffer` - The new GPU buffer.
NSTDAPI NSTDGLBuffer nstd_gl_buffer_new(const NSTDSlice *const bytes, const NSTDGLDevice device);

/// Frees a GPU buffer.
/// Parameters:
///     `NSTDGLBuffer *const buffer` - The GPU buffer.
NSTDAPI void nstd_gl_buffer_free(NSTDGLBuffer *const buffer);

NSTDCPPEND
#endif
