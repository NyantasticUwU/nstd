#ifndef NSTD_GL_DEF_H_INCLUDED
#define NSTD_GL_DEF_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"

/// Represents a color.
// Must match https://docs.rs/wgpu/0.12.0/wgpu/struct.Color.html.
typedef struct
{
    // Red color value.
    NSTDFloat64 r;
    // Green color value.
    NSTDFloat64 g;
    // Blue color value.
    NSTDFloat64 b;
    // Alpha color value.
    NSTDFloat64 a;
} NSTDGLColor;

/// Represents a graphics backend.
typedef enum
{
    /// An unknown graphics backend.
    NSTD_GL_BACKEND_UNKNOWN,
    /// Vulkan.
    NSTD_GL_BACKEND_VULKAN,
    /// Metal.
    NSTD_GL_BACKEND_METAL,
    /// Direct3D 12.
    NSTD_GL_BACKEND_DX12,
    /// Direct3D 11.
    NSTD_GL_BACKEND_DX11,
    /// OpenGL.
    NSTD_GL_BACKEND_GL,
    /// Web based GPU.
    NSTD_GL_BACKEND_WEBGPU
} NSTDGLBackend;

/// Represents a state's presentation mode.
typedef enum
{
    /// `wgpu`'s presentation engine will request drawing immediately.
    NSTD_GL_PRESENTATION_MODE_IMMEDIATE,
    /// Waits for the vertical blanking period, but frames are submitted immediately.
    NSTD_GL_PRESENTATION_MODE_MAILBOX,
    /// Waits for the vertical blanking period, and frames are
    /// submitted with the monitor's referesh rate.
    NSTD_GL_PRESENTATION_MODE_FIFO
} NSTDGLPresentationMode;

/// Represents a power preference.
typedef enum
{
    /// Use the default power preference.
    NSTD_GL_POWER_PREFERENCE_DEFAULT,
    /// Use low GPU power.
    NSTD_GL_POWER_PREFERENCE_LOW,
    /// Use high GPU power.
    NSTD_GL_POWER_PREFERENCE_HIGH
} NSTDGLPowerPreference;

/// Represents a device type.
typedef enum
{
    /// An unknown device type.
    NSTD_GL_DEVICE_TYPE_UNKNOWN,
    /// `wgpu`'s integrated GPU.
    NSTD_GL_DEVICE_TYPE_INTEGRATED_GPU,
    /// A physical GPU.
    NSTD_GL_DEVICE_TYPE_DISCRETE_GPU,
    /// A virtual/hosted GPU.
    NSTD_GL_DEVICE_TYPE_VIRTUAL_GPU,
    /// CPU/Software rendering.
    NSTD_GL_DEVICE_TYPE_CPU
} NSTDGLDeviceType;

#endif
