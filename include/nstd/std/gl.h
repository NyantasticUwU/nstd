#ifndef NSTD_STD_GL_H_INCLUDED
#define NSTD_STD_GL_H_INCLUDED
#include "../core/def.h"
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
    NSTDSize vendor;
    NSTDSize device;
    NSTDGLDeviceType device_type;
    NSTDGLBackend backend;
} NSTDGLDeviceInfo;

/// Creates a new GL state.
/// Parameters:
///     `NSTDWindow window` - The window in which the GL state will live in.
/// Returns: `NSTDGLState state` - The new GL state.
NSTDAPI NSTDGLState nstd_std_gl_state_new(NSTDWindow window);

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
///     `const NSTDByte *const data` - Raw spirv data.
///     `const NSTDSize size` - Number of bytes of spirv data.
///     `NSTDGLDevice device` - The device to create the shader module on.
/// Returns: `NSTDGLShaderModule shader` - The new shader module.
NSTDAPI NSTDGLShaderModule nstd_std_gl_shader_module_new(
    const NSTDByte *const data,
    const NSTDSize size,
    NSTDGLDevice device);

/// Frees a shader module.
/// Parameters:
///     `NSTDGLShaderModule *shader` - Pointer to a shader module.
NSTDAPI void nstd_std_gl_shader_module_free(NSTDGLShaderModule *shader);

/// Creates a new render pipeline from a vertex and fragment shader.
/// Parameters:
///     `NSTDGLShaderModule vert` - The vertex shader module.
///     `NSTDGLShaderModule frag` - The fragment shader module.
///     `NSTDGLDevice device` - The device to create the render pipeline on.
///     `NSTDGLSurfaceConfiguration config` - The surface configuration.
/// Returns: `NSTDGLRenderPipeline pipeline` - The new render pipeline.
NSTDAPI NSTDGLRenderPipeline nstd_std_gl_render_pipeline_new(
    NSTDGLShaderModule vert,
    NSTDGLShaderModule frag,
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

/// Draws primitives from active vertex buffers.
/// Parameters:
///     `NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDUInt32 verticies` - Number of verticies to draw.
///     `const NSTDUInt32 instances` - Number of instnaces.
NSTDAPI void nstd_std_gl_render_pass_draw(
    NSTDGLRenderPass render_pass,
    const NSTDUInt32 verticies,
    const NSTDUInt32 instances);

/// Frees an `NSTDGLDeviceInfo` object.
/// Parameters:
///     `NSTDGLDeviceInfo *const device_info` - Pointer to an `NSTDGLDeviceInfo` object.
NSTDAPI void nstd_std_gl_device_info_free(NSTDGLDeviceInfo *const device_info);

#ifdef __cplusplus
}
#endif
#endif
