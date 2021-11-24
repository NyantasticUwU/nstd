#ifndef NSTD_STD_GL_H_INCLUDED
#define NSTD_STD_GL_H_INCLUDED
#include "../core/def.h"
#include "../core/slice.h"
#include "def.h"
#include "gui.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a color.
typedef struct
{
    double r;
    double g;
    double b;
    double a;
} NSTDGLColor;

/// Represents a graphical surface.
typedef void *NSTDGLSurface;

/// Represents a surface config.
typedef void *NSTDGLSurfaceConfiguration;

/// Represents a handle to a physical graphics device.
typedef void *NSTDGLDeviceHandle;

/// Represents a graphics device.
typedef void *NSTDGLDevice;

/// Represents a graphics device command queue.
typedef void *NSTDGLQueue;

/// Represents a shader module.
typedef void *NSTDGLShaderModule;

/// Represents a render pipeline.
typedef void *NSTDGLRenderPipeline;

/// Represents a render pass object.
typedef void *NSTDGLRenderPass;

/// Represents a GPU buffer.
typedef void *NSTDGLBuffer;

/// Represents a GL state.
typedef struct
{
    NSTDGLSurface surface;
    NSTDGLSurfaceConfiguration config;
    NSTDGLDeviceHandle device_handle;
    NSTDGLDevice device;
    NSTDGLQueue queue;
    NSTDWindowSize size;
    NSTDGLColor clear_color;
} NSTDGLState;

/// Represents a graphics backend.
typedef enum
{
    NSTD_GL_BACKEND_UNKNOWN,
    NSTD_GL_BACKEND_VULKAN,
    NSTD_GL_BACKEND_METAL,
    NSTD_GL_BACKEND_DX12,
    NSTD_GL_BACKEND_DX11,
    NSTD_GL_BACKEND_GL,
    NSTD_GL_BACKEND_WEBGPU
} NSTDGLBackend;

/// Represents a device type.
typedef enum
{
    NSTD_GL_DEVICE_TYPE_UNKNOWN,
    NSTD_GL_DEVICE_TYPE_INTEGRATED_GPU,
    NSTD_GL_DEVICE_TYPE_DISCRETE_GPU,
    NSTD_GL_DEVICE_TYPE_VIRTUAL_GPU,
    NSTD_GL_DEVICE_TYPE_CPU
} NSTDGLDeviceType;

/// Contains information on a device.
typedef struct
{
    char *name;
    NSTDUSize vendor;
    NSTDUSize device;
    NSTDGLDeviceType device_type;
    NSTDGLBackend backend;
} NSTDGLDeviceInfo;

/// Represents a state's presentation mode.
typedef enum
{
    NSTD_GL_PRESENTATION_MODE_IMMEDIATE,
    NSTD_GL_PRESENTATION_MODE_MAILBOX,
    NSTD_GL_PRESENTATION_MODE_FIFO
} NSTDGLPresentationMode;

/// Represents a power preference.
typedef enum
{
    NSTD_GL_POWER_PREFERENCE_DEFAULT,
    NSTD_GL_POWER_PREFERENCE_LOW,
    NSTD_GL_POWER_PREFERENCE_HIGH
} NSTDGLPowerPreference;

/// Configures a GL state upon creation.
/// For `backend`, `NSTD_GL_BACKEND_UNKNOWN` will pick a default backend to use.
typedef struct
{
    NSTDGLBackend backend;
    NSTDGLPowerPreference power_preference;
    NSTDGLPresentationMode presentation_mode;
} NSTDGLStateDescriptor;

/// Represents a vertex format when sending data to the vertex shader.
typedef enum
{
    NSTD_GL_VERTEX_FORMAT_UINT8X2,
    NSTD_GL_VERTEX_FORMAT_UINT8X4,
    NSTD_GL_VERTEX_FORMAT_INT8X2,
    NSTD_GL_VERTEX_FORMAT_INT8X4,
    NSTD_GL_VERTEX_FORMAT_UNORM8X2,
    NSTD_GL_VERTEX_FORMAT_UNORM8X4,
    NSTD_GL_VERTEX_FORMAT_NORM8X2,
    NSTD_GL_VERTEX_FORMAT_NORM8X4,
    NSTD_GL_VERTEX_FORMAT_UINT16X2,
    NSTD_GL_VERTEX_FORMAT_UINT16X4,
    NSTD_GL_VERTEX_FORMAT_INT16X2,
    NSTD_GL_VERTEX_FORMAT_INT16X4,
    NSTD_GL_VERTEX_FORMAT_UNORM16X2,
    NSTD_GL_VERTEX_FORMAT_UNORM16X4,
    NSTD_GL_VERTEX_FORMAT_NORM16X2,
    NSTD_GL_VERTEX_FORMAT_NORM16X4,
    NSTD_GL_VERTEX_FORMAT_FLOAT16X2,
    NSTD_GL_VERTEX_FORMAT_FLOAT16X4,
    NSTD_GL_VERTEX_FORMAT_FLOAT32,
    NSTD_GL_VERTEX_FORMAT_FLOAT32X2,
    NSTD_GL_VERTEX_FORMAT_FLOAT32X3,
    NSTD_GL_VERTEX_FORMAT_FLOAT32X4,
    NSTD_GL_VERTEX_FORMAT_UINT32,
    NSTD_GL_VERTEX_FORMAT_UINT32X2,
    NSTD_GL_VERTEX_FORMAT_UINT32X3,
    NSTD_GL_VERTEX_FORMAT_UINT32X4,
    NSTD_GL_VERTEX_FORMAT_INT32,
    NSTD_GL_VERTEX_FORMAT_INT32X2,
    NSTD_GL_VERTEX_FORMAT_INT32X3,
    NSTD_GL_VERTEX_FORMAT_INT32X4,
    NSTD_GL_VERTEX_FORMAT_FLOAT64,
    NSTD_GL_VERTEX_FORMAT_FLOAT64X2,
    NSTD_GL_VERTEX_FORMAT_FLOAT64X3,
    NSTD_GL_VERTEX_FORMAT_FLOAT64X4
} NSTDGLVertexFormat;

/// Represents an index format when drawing indexed verticies.
typedef enum
{
    NSTD_GL_INDEX_FORMAT_UINT16,
    NSTD_GL_INDEX_FORMAT_UINT32,
} NSTDGLIndexFormat;

/// Represents a vertex attribute.
/// NOTE: This struct must directly mirror `VertexAttribute` defined by wgpu in terms of size,
/// alignment, and order of fields.
typedef struct
{
    NSTDGLVertexFormat format;
    NSTDUInt64 offset;
    NSTDUInt32 location;
} NSTDGLVertexAttribute;

/// Represents a vertex stepping mode.
typedef enum
{
    NSTD_GL_VERTEX_STEP_MODE_VERTEX,
    NSTD_GL_VERTEX_STEP_MODE_INSTANCE,
} NSTDGLVertexStepMode;

/// Represents a vertex buffer layout.
/// `attributes` - `&mut [NSTDGLVertexAttribute]`.
typedef struct
{
    NSTDUInt64 stride;
    NSTDGLVertexStepMode step_mode;
    NSTDSlice attributes;
} NSTDGLVertexBufferLayout;

/// Creates a new GL state.
/// Parameters:
///     `NSTDWindow window` - The window in which the GL state will live in.
///     `const NSTDGLStateDescriptor descriptor` - Configures the state.
/// Returns: `NSTDGLState state` - The new GL state.
NSTDAPI NSTDGLState nstd_std_gl_state_new(
    NSTDWindow window,
    const NSTDGLStateDescriptor descriptor);

/// Pushes the current frame to the display.
/// Parameters:
///     `const NSTDGLState *const state` - The GL state.
///     `void(*callback)(NSTDGLRenderPass)` - Manipulates the render pass.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_gl_state_render(
    const NSTDGLState *const state,
    void(*callback)(NSTDGLRenderPass));

/// Resizes a GL state's context.
/// Parameters:
///     `NSTDGLState *const state` - The GL state.
///     `const NSTDWindowSize *const new_size` - The new context size.
NSTDAPI void nstd_std_gl_state_resize(
    NSTDGLState *const state,
    const NSTDWindowSize *const new_size);

/// Frees and destroys a GL state.
/// Parameters:
///     `NSTDGLState *const state` - The GL state.
NSTDAPI void nstd_std_gl_state_free(NSTDGLState *const state);

/// Retrieves info on a device.
/// Parameters:
///     `NSTDGLDeviceHandle device_handle` - Handle to a device.
/// Returns: `NSTDGLDeviceInfo device_info` - Contains information about a device.
NSTDAPI NSTDGLDeviceInfo nstd_std_gl_device_handle_get_info(NSTDGLDeviceHandle device_handle);

/// Creates a new shader module.
/// Parameters:
///     `const NSTDSlice *const data` - Raw spirv data.
///     `NSTDGLDevice device` - The device to create the shader module on.
/// Returns: `NSTDGLShaderModule shader` - The new shader module.
NSTDAPI NSTDGLShaderModule nstd_std_gl_shader_module_new(
    const NSTDSlice *const data,
    NSTDGLDevice device);

/// Frees a shader module.
/// Parameters:
///     `NSTDGLShaderModule *shader` - Pointer to a shader module.
NSTDAPI void nstd_std_gl_shader_module_free(NSTDGLShaderModule *shader);

/// Creates a new render pipeline from a vertex and fragment shader.
/// Parameters:
///     `NSTDGLShaderModule vert` - The vertex shader module.
///     `NSTDGLShaderModule frag` - The fragment shader module.
///     `const NSTDSlice *const buffers` - A slice of `NSTDGLVertexBufferLayout`s.
///     `NSTDGLDevice device` - The device to create the render pipeline on.
///     `NSTDGLSurfaceConfiguration config` - The surface configuration.
/// Returns: `NSTDGLRenderPipeline pipeline` - The new render pipeline.
NSTDAPI NSTDGLRenderPipeline nstd_std_gl_render_pipeline_new(
    NSTDGLShaderModule vert,
    NSTDGLShaderModule frag,
    const NSTDSlice *const buffers,
    NSTDGLDevice device,
    NSTDGLSurfaceConfiguration config);

/// Frees a render pipeline.
/// Parameters:
///     `NSTDGLRenderPipeline *pipeline` - Pointer to a render pipeline.
NSTDAPI void nstd_std_gl_render_pipeline_free(NSTDGLRenderPipeline *pipeline);

/// Sets a render pipeline for a render pass.
/// Parameters:
///     `NSTDGLRenderPass render_pass` - The render pass.
///     `NSTDGLRenderPipeline pipeline` - The render pipeline.
NSTDAPI void nstd_std_gl_render_pass_set_pipeline(
    NSTDGLRenderPass render_pass,
    NSTDGLRenderPipeline pipeline);

/// Sets a render pass' vertex buffer.
/// Parameters:
///     `NSTDGLRenderPass render_pass` - The render pass.
///     `NSTDGLBuffer buffer` - The GPU vertex buffer.
///     `const NSTDUInt32 slot` - The buffer slot (the index of the buffer layout).
NSTDAPI void nstd_std_gl_render_pass_set_vertex_buffer(
    NSTDGLRenderPass render_pass,
    NSTDGLBuffer buffer,
    const NSTDUInt32 slot);

/// Sets a render pass' index buffer.
/// Parameters:
///     `NSTDGLRenderPass render_pass` - The render pass.
///     `NSTDGLBuffer buffer` - The GPU index buffer.
///     `NSTDGLIndexFormat format` - The index format of the buffer.
NSTDAPI void nstd_std_gl_render_pass_set_index_buffer(
    NSTDGLRenderPass render_pass,
    NSTDGLBuffer buffer,
    NSTDGLIndexFormat format);

/// Draws primitives from active vertex buffers.
/// Parameters:
///     `NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDUInt32 verticies` - Number of verticies to draw.
///     `const NSTDUInt32 instances` - Number of instnaces.
NSTDAPI void nstd_std_gl_render_pass_draw(
    NSTDGLRenderPass render_pass,
    const NSTDUInt32 verticies,
    const NSTDUInt32 instances);

/// Draws primitives from active vertex and index buffers.
/// Parameters:
///     `NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDUInt32 indicies` - The indicies to draw.
///     `const NSTDUInt32 instances` - The instances to draw.
///     `const NSTDInt32 base` - The base vertex.
NSTDAPI void nstd_std_gl_render_pass_draw_indexed(
    NSTDGLRenderPass render_pass,
    const NSTDUInt32 indicies,
    const NSTDUInt32 instances,
    const NSTDInt32 base);

/// Frees an `NSTDGLDeviceInfo` object.
/// Parameters:
///     `NSTDGLDeviceInfo *const device_info` - Pointer to an `NSTDGLDeviceInfo` object.
NSTDAPI void nstd_std_gl_device_info_free(NSTDGLDeviceInfo *const device_info);

/// Creates a new GPU buffer.
/// Parameters:
///     `const NSTDSlice *const bytes` - The bytes to send to the GPU.
///     `NSTDGLDevice device` - The device to create the buffer on.
/// Returns: `NSTDGLBuffer buffer` - The new GPU buffer.
NSTDAPI NSTDGLBuffer nstd_std_gl_buffer_new(const NSTDSlice *const bytes, NSTDGLDevice device);

/// Frees a GPU buffer.
/// Parameters:
///     `NSTDGLBuffer *const buffer` - The GPU buffer.
NSTDAPI void nstd_std_gl_buffer_free(NSTDGLBuffer *const buffer);


#ifdef __cplusplus
}
#endif
#endif
