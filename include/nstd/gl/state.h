#ifndef NSTD_GL_STATE_H_INCLUDED
#define NSTD_GL_STATE_H_INCLUDED
#include "../core/def.h"
#include "../gui.h"
#include "../nstd.h"
#include "command/buffer.h"
#include "def.h"
#include "device.h"
#include "render/pass.h"
#include "render/pipeline.h"
#include "surface.h"
#include "texture.h"
NSTDCPPSTART

/// Represents a GL state.
typedef struct
{
    /// The surface to draw on.
    NSTDGLSurface surface;
    /// The surface configuration.
    NSTDGLSurfaceConfig config;
    /// A handle to the drawing device.
    NSTDGLDeviceHandle device_handle;
    /// The drawing device.
    NSTDGLDevice device;
    /// The window's clear color.
    NSTDGLColor clear_color;
} NSTDGLState;

/// Creates a new GL state.
/// NOTE: `surface`, `config`, `device_handle` and `device` are all freed once the state is freed.
/// Parameters:
///     `const NSTDGLSurface surface` - The surface that the state will use.
///     `const NSTDGLSurfaceConfig config` - The surface configuration.
///     `const NSTDGLDeviceHandle device_handle` - The device handle to create the device with.
///     `const NSTDGLDevice device` - The drawing device.
/// Returns: `NSTDGLState state` - The new GL state.
NSTDAPI NSTDGLState nstd_gl_state_new(
    const NSTDGLSurface surface,
    const NSTDGLSurfaceConfig config,
    const NSTDGLDeviceHandle device_handle,
    const NSTDGLDevice device);

/// Pushes the current frame to the display.
/// Note: This function frees `command_buffer`, and `surface_texture`.
/// Parameters:
///     `const NSTDGLState *const state` - The GL state.
///     `NSTDGLCommandBuffer *const command_buffer` - A device command buffer.
///     `NSTDGLSurfaceTexture *const surface_texture` - The surface texture to use, this is freed.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_gl_state_render(
    const NSTDGLState *const state,
    NSTDGLCommandBuffer *const command_buffer,
    NSTDGLSurfaceTexture *const surface_texture);

/// Resizes a GL state's context.
/// Parameters:
///     `NSTDGLState *const state` - The GL state.
///     `const NSTDWindowSize *const new_size` - The new context size.
NSTDAPI void nstd_gl_state_resize(NSTDGLState *const state, const NSTDWindowSize *const new_size);

/// Frees and destroys a GL state.
/// Parameters:
///     `NSTDGLState *const state` - The GL state.
NSTDAPI void nstd_gl_state_free(NSTDGLState *const state);

NSTDCPPEND
#endif
