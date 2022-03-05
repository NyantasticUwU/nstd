use crate::{
    core::def::NSTDErrorCode,
    gl::{
        command::buffer::NSTDGLCommandBuffer,
        def::NSTDGLColor,
        device::{handle::NSTDGLDeviceHandle, NSTDGLDevice},
        surface::{config::NSTDGLSurfaceConfig, NSTDGLSurface},
        texture::NSTDGLSurfaceTexture,
    },
    gui::def::NSTDWindowSize,
};

/// Represents a GL state.
#[repr(C)]
#[derive(Debug)]
pub struct NSTDGLState {
    /// The surface to draw on.
    pub surface: NSTDGLSurface,
    /// The surface configuration.
    pub config: NSTDGLSurfaceConfig,
    /// A handle to the drawing device.
    pub device_handle: NSTDGLDeviceHandle,
    /// The drawing device.
    pub device: NSTDGLDevice,
    /// The window's clear color.
    pub clear_color: NSTDGLColor,
}
impl Default for NSTDGLState {
    #[inline]
    fn default() -> Self {
        Self {
            surface: std::ptr::null_mut(),
            config: std::ptr::null_mut(),
            device_handle: std::ptr::null_mut(),
            device: NSTDGLDevice::default(),
            clear_color: NSTDGLColor::default(),
        }
    }
}

/// Creates a new GL state.
/// NOTE: `surface`, `config`, `device_handle` and `device` are all freed once the state is freed.
/// Parameters:
///     `const NSTDGLSurface surface` - The surface that the state will use.
///     `const NSTDGLSurfaceConfig config` - The surface configuration.
///     `const NSTDGLDeviceHandle device_handle` - The device handle to create the device with.
///     `const NSTDGLDevice device` - The drawing device.
/// Returns: `NSTDGLState state` - The new GL state.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_state_new(
    surface: NSTDGLSurface,
    config: NSTDGLSurfaceConfig,
    device_handle: NSTDGLDeviceHandle,
    device: NSTDGLDevice,
) -> NSTDGLState {
    // Configuring the surface.
    (*surface).configure(&*device.raw, &*config);
    // Constructing the state.
    NSTDGLState {
        surface,
        config,
        device_handle,
        device,
        clear_color: NSTDGLColor::default(),
    }
}

/// Pushes the current frame to the display.
/// Note: This function frees `command_buffer`, and `surface_texture`.
/// Parameters:
///     `const NSTDGLState *const state` - The GL state.
///     `NSTDGLCommandBuffer *const command_buffer` - A device command buffer.
///     `NSTDGLSurfaceTexture *const surface_texture` - The surface texture to use, this is freed.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_state_render(
    state: &NSTDGLState,
    command_buffer: &mut NSTDGLCommandBuffer,
    surface_texture: &mut NSTDGLSurfaceTexture,
) -> NSTDErrorCode {
    // Submitting the command buffer and presenting the texture.
    (*state.device.command_queue).submit(std::iter::once(*Box::from_raw(*command_buffer)));
    Box::from_raw(*surface_texture).present();
    // Setting old data pointers to null.
    *command_buffer = std::ptr::null_mut();
    *surface_texture = std::ptr::null_mut();
    0
}

/// Resizes a GL state's context.
/// Parameters:
///     `NSTDGLState *const state` - The GL state.
///     `const NSTDWindowSize *const new_size` - The new context size.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_state_resize(state: &mut NSTDGLState, new_size: &NSTDWindowSize) {
    if new_size.width > 0 && new_size.height > 0 {
        (*state.config).width = new_size.width;
        (*state.config).height = new_size.height;
        (*state.surface).configure(&*state.device.raw, &*state.config);
    }
}

/// Frees and destroys a GL state.
/// Parameters:
///     `NSTDGLState *const state` - The GL state.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_state_free(state: &mut NSTDGLState) {
    crate::gl::surface::nstd_gl_surface_free(&mut state.surface);
    crate::gl::surface::config::nstd_gl_surface_config_free(&mut state.config);
    crate::gl::device::handle::nstd_gl_device_handle_free(&mut state.device_handle);
    crate::gl::device::nstd_gl_device_free(&mut state.device);
}
