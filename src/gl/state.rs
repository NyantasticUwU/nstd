use crate::{
    core::def::NSTDErrorCode,
    gl::{
        def::{NSTDGLBackend, NSTDGLColor, NSTDGLPowerPreference, NSTDGLPresentationMode},
        device::{NSTDGLDevice, NSTDGLDeviceHandle, NSTDGLQueue},
        pipeline::NSTDGLRenderPass,
        surface::{NSTDGLSurface, NSTDGLSurfaceConfiguration},
    },
    gui::{def::NSTDWindowSize, NSTDWindow},
};
use wgpu::{
    CommandEncoderDescriptor, DeviceDescriptor, Instance, LoadOp, Operations,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, SurfaceConfiguration,
    TextureUsages, TextureViewDescriptor,
};

/// Represents a GL state.
#[repr(C)]
#[derive(Debug)]
pub struct NSTDGLState {
    /// The surface to draw on.
    pub surface: NSTDGLSurface,
    /// The surface configuration.
    pub config: NSTDGLSurfaceConfiguration,
    /// A handle to the drawing device.
    pub device_handle: NSTDGLDeviceHandle,
    /// The drawing device.
    pub device: NSTDGLDevice,
    /// The device's command queue.
    pub queue: NSTDGLQueue,
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
            device: std::ptr::null_mut(),
            queue: std::ptr::null_mut(),
            clear_color: NSTDGLColor::default(),
        }
    }
}

/// Configures a GL state upon creation.
/// For `backend`, `NSTD_GL_BACKEND_UNKNOWN` will pick a default backend to use.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDGLStateDescriptor {
    /// The graphics backend to use.
    pub backend: NSTDGLBackend,
    /// The amount of GPU power to be used.
    pub power_preference: NSTDGLPowerPreference,
    /// The way frames will be presented to the display.
    pub presentation_mode: NSTDGLPresentationMode,
}

/// Creates a new GL state.
/// Parameters:
///     `const NSTDWindow window` - The window in which the GL state will live in.
///     `const NSTDGLStateDescriptor descriptor` - Configures the state.
/// Returns: `NSTDGLState state` - The new GL state.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_state_new(
    window: NSTDWindow,
    descriptor: NSTDGLStateDescriptor,
) -> NSTDGLState {
    // Creating a wgpu instance
    let instance = Instance::new(descriptor.backend.into());
    // Creating a surface on the window.
    let surface = instance.create_surface(&*window);
    // Getting the drawing device and it's command queue.
    let adapter_options = RequestAdapterOptions {
        power_preference: descriptor.power_preference.into(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    };
    let adapter = match futures::executor::block_on(instance.request_adapter(&adapter_options)) {
        Some(adapter) => adapter,
        _ => return NSTDGLState::default(),
    };
    let dqfut = adapter.request_device(&DeviceDescriptor::default(), None);
    let (device, queue) = match futures::executor::block_on(dqfut) {
        Ok((device, queue)) => (device, queue),
        _ => return NSTDGLState::default(),
    };
    // Configuring the surface.
    let size = crate::gui::nstd_gui_window_get_client_size(window);
    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: match surface.get_preferred_format(&adapter) {
            Some(format) => format,
            _ => return NSTDGLState::default(),
        },
        width: size.width,
        height: size.height,
        present_mode: descriptor.presentation_mode.into(),
    };
    surface.configure(&device, &config);
    // Constructing the state.
    NSTDGLState {
        surface: Box::into_raw(Box::new(surface)),
        config: Box::into_raw(Box::new(config)),
        device_handle: Box::into_raw(Box::new(adapter)),
        device: Box::into_raw(Box::new(device)),
        queue: Box::into_raw(Box::new(queue)),
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
    let mut encoder = (*state.device).create_command_encoder(&CommandEncoderDescriptor::default());
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
    (*state.queue).submit(std::iter::once(encoder.finish()));
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
        (*state.surface).configure(&*state.device, &*state.config);
    }
}

/// Frees and destroys a GL state.
/// Parameters:
///     `NSTDGLState *const state` - The GL state.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_state_free(state: &mut NSTDGLState) {
    Box::from_raw(state.surface);
    Box::from_raw(state.config);
    Box::from_raw(state.device_handle);
    Box::from_raw(state.device);
    Box::from_raw(state.queue);
    state.surface = std::ptr::null_mut();
    state.config = std::ptr::null_mut();
    state.device_handle = std::ptr::null_mut();
    state.device = std::ptr::null_mut();
    state.queue = std::ptr::null_mut();
}
