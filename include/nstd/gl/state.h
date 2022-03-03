#ifndef NSTD_GL_STATE_H_INCLUDED
#define NSTD_GL_STATE_H_INCLUDED
#include "../core/def.h"
#include "../gui.h"
#include "../nstd.h"
#include "def.h"
#include "device.h"
#include "instance.h"
#include "pipeline.h"
#include "surface.h"
#include "texture.h"
NSTDCPPSTART

/// Represents a GL state.
typedef struct
{
    /// The surface to draw on.
    NSTDGLSurface surface;
    /// The surface configuration.
    NSTDGLSurfaceConfiguration config;
    /// A handle to the drawing device.
    NSTDGLDeviceHandle device_handle;
    /// The drawing device.
    NSTDGLDevice device;
    /// The window's clear color.
    NSTDGLColor clear_color;
} NSTDGLState;

/// Creates a new GL state.
/// NOTE: `surface`, `device_handle` and `device` are freed once the state is freed.
/// Parameters:
///     `const NSTDWindow window` - The window in which the GL state will live in.
///     `const NSTDGLSurface surface` - The surface that the state will use.
///     `const NSTDGLDeviceHandle device_handle` - The device handle to create the device with.
///     `const NSTDGLDevice device` - The drawing device.
///     `const NSTDGLPresentationMode presentation_mode` - The presentation mode.
///     `const NSTDGLTextureFormat texture_format` - The texture format to use for the surface.
/// Returns: `NSTDGLState state` - The new GL state.
NSTDAPI NSTDGLState nstd_gl_state_new(
    const NSTDWindow window,
    const NSTDGLSurface surface,
    const NSTDGLDeviceHandle device_handle,
    const NSTDGLDevice device,
    const NSTDGLPresentationMode presentation_mode,
    const NSTDGLTextureFormat texture_format);

/// Pushes the current frame to the display.
/// Parameters:
///     `const NSTDGLState *const state` - The GL state.
///     `void(*callback)(NSTDGLRenderPass)` - Manipulates the render pass.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_gl_state_render(
    const NSTDGLState *const state,
    void(*callback)(NSTDGLRenderPass));

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
