use crate::{
    core::def::NSTDErrorCode,
    gl::{
        def::NSTDGLColor,
        device::{NSTDGLDevice, NSTDGLDeviceHandle},
        pipeline::NSTDGLRenderPass,
        surface::{config::NSTDGLSurfaceConfig, NSTDGLSurface},
    },
    gui::def::NSTDWindowSize,
};
use wgpu::{
    CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    TextureViewDescriptor,
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
/// Parameters:
///     `const NSTDGLState *const state` - The GL state.
///     `void(*callback)(NSTDGLRenderPass)` - Manipulates the render pass.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_state_render(
    state: &NSTDGLState,
    callback: extern "C" fn(NSTDGLRenderPass),
) -> NSTDErrorCode {
    // Getting a view to the texture to be displayed.
    let output = match (*state.surface).get_current_texture() {
        Ok(output) => output,
        _ => return 1,
    };
    let view_options = TextureViewDescriptor::default();
    let view = output.texture.create_view(&view_options);
    // Create a render pass.
    let encoder_desc = CommandEncoderDescriptor::default();
    let mut encoder = (*state.device.raw).create_command_encoder(&encoder_desc);
    {
        let render_pass_descriptor = RenderPassDescriptor {
            label: None,
            color_attachments: &[RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(state.clear_color),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        };
        let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);
        callback(&mut render_pass);
    }
    // Finish and present the texture to the display.
    (*state.device.command_queue).submit(std::iter::once(encoder.finish()));
    output.present();
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
    crate::gl::device::nstd_gl_device_handle_free(&mut state.device_handle);
    crate::gl::device::nstd_gl_device_free(&mut state.device);
    Box::from_raw(state.config);
    state.config = std::ptr::null_mut();
}
