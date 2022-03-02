#ifndef NSTD_GL_STATE_H_INCLUDED
#define NSTD_GL_STATE_H_INCLUDED
#include "../core/def.h"
#include "../gui.h"
#include "../nstd.h"
#include "def.h"
#include "device.h"
#include "pipeline.h"
#include "surface.h"

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
    /// The device's command queue.
    NSTDGLQueue queue;
    /// The window's clear color.
    NSTDGLColor clear_color;
} NSTDGLState;

/// Configures a GL state upon creation.
/// For `backend`, `NSTD_GL_BACKEND_UNKNOWN` will pick a default backend to use.
typedef struct
{
    /// The graphics backend to use.
    NSTDGLBackend backend;
    /// The amount of GPU power to be used.
    NSTDGLPowerPreference power_preference;
    /// The way frames will be presented to the display.
    NSTDGLPresentationMode presentation_mode;
} NSTDGLStateDescriptor;

/// Creates a new GL state.
/// Parameters:
///     `const NSTDWindow window` - The window in which the GL state will live in.
///     `const NSTDGLStateDescriptor descriptor` - Configures the state.
/// Returns: `NSTDGLState state` - The new GL state.
NSTDAPI NSTDGLState nstd_gl_state_new(
    const NSTDWindow window,
    const NSTDGLStateDescriptor descriptor);

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

#endif
