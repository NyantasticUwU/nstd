use futures::executor;
use nstd_gui::{NSTDWindow, NSTDWindowSize};
use std::{
    ffi::CString,
    os::raw::{c_char, c_double, c_int},
    ptr,
};
use wgpu::{
    Adapter, Backend, Backends, BlendState, Color, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, Device, DeviceDescriptor, DeviceType, Face, FragmentState, FrontFace,
    Instance, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode,
    PowerPreference, PresentMode, PrimitiveState, PrimitiveTopology, Queue, RenderPass,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    RequestAdapterOptions, ShaderModule, ShaderModuleDescriptor, ShaderSource, Surface,
    SurfaceConfiguration, TextureUsages, TextureViewDescriptor, VertexState,
};

/// Represents a color.
#[repr(C)]
#[derive(Default)]
pub struct NSTDGLColor {
    pub r: c_double,
    pub g: c_double,
    pub b: c_double,
    pub a: c_double,
}

/// Represents a graphical surface.
pub type NSTDGLSurface = *mut Surface;

/// Represents a surface config.
pub type NSTDGLSurfaceConfiguration = *mut SurfaceConfiguration;

/// Represents a handle to a physical graphics device.
pub type NSTDGLDeviceHandle = *mut Adapter;

/// Represents a graphics device.
pub type NSTDGLDevice = *mut Device;

/// Represents a graphics device command queue.
pub type NSTDGLQueue = *mut Queue;

/// Represents a shader module.
pub type NSTDGLShaderModule = *mut ShaderModule;

/// Represents a render pipeline.
pub type NSTDGLRenderPipeline = *mut RenderPipeline;

/// Represents a render pass object.
pub type NSTDGLRenderPass<'a> = *mut RenderPass<'a>;

/// Represents a GL state.
#[repr(C)]
pub struct NSTDGLState {
    pub surface: NSTDGLSurface,
    pub config: NSTDGLSurfaceConfiguration,
    pub device_handle: NSTDGLDeviceHandle,
    pub device: NSTDGLDevice,
    pub queue: NSTDGLQueue,
    pub size: NSTDWindowSize,
    pub clear_color: NSTDGLColor,
}
impl Default for NSTDGLState {
    #[inline]
    fn default() -> Self {
        Self {
            surface: ptr::null_mut(),
            config: ptr::null_mut(),
            device_handle: ptr::null_mut(),
            device: ptr::null_mut(),
            queue: ptr::null_mut(),
            size: NSTDWindowSize::new(0, 0),
            clear_color: NSTDGLColor::default(),
        }
    }
}

/// Represents a graphics backend.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDGLBackend {
    NSTD_GL_BACKEND_UNKNOWN,
    NSTD_GL_BACKEND_VULKAN,
    NSTD_GL_BACKEND_METAL,
    NSTD_GL_BACKEND_DX12,
    NSTD_GL_BACKEND_DX11,
    NSTD_GL_BACKEND_GL,
    NSTD_GL_BACKEND_WEBGPU,
}
impl Into<Backends> for NSTDGLBackend {
    #[inline]
    fn into(self) -> Backends {
        match self {
            Self::NSTD_GL_BACKEND_UNKNOWN => Backends::all(),
            Self::NSTD_GL_BACKEND_VULKAN => Backends::VULKAN,
            Self::NSTD_GL_BACKEND_METAL => Backends::METAL,
            Self::NSTD_GL_BACKEND_DX11 => Backends::DX11,
            Self::NSTD_GL_BACKEND_DX12 => Backends::DX12,
            Self::NSTD_GL_BACKEND_GL => Backends::GL,
            Self::NSTD_GL_BACKEND_WEBGPU => Backends::BROWSER_WEBGPU,
        }
    }
}

/// Represents a device type.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDGLDeviceType {
    NSTD_GL_DEVICE_TYPE_UNKNOWN,
    NSTD_GL_DEVICE_TYPE_INTEGRATED_GPU,
    NSTD_GL_DEVICE_TYPE_DISCRETE_GPU,
    NSTD_GL_DEVICE_TYPE_VIRTUAL_GPU,
    NSTD_GL_DEVICE_TYPE_CPU,
}

/// Contains information on a device.
#[repr(C)]
pub struct NSTDGLDeviceInfo {
    pub name: *mut c_char,
    pub vendor: usize,
    pub device: usize,
    pub device_type: NSTDGLDeviceType,
    pub backend: NSTDGLBackend,
}

/// Represents a state's presentation mode.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDGLPresentationMode {
    NSTD_GL_PRESENTATION_MODE_IMMEDIATE,
    NSTD_GL_PRESENTATION_MODE_MAILBOX,
    NSTD_GL_PRESENTATION_MODE_FIFO,
}
impl Into<PresentMode> for NSTDGLPresentationMode {
    #[inline]
    fn into(self) -> PresentMode {
        match self {
            Self::NSTD_GL_PRESENTATION_MODE_IMMEDIATE => PresentMode::Immediate,
            Self::NSTD_GL_PRESENTATION_MODE_MAILBOX => PresentMode::Mailbox,
            Self::NSTD_GL_PRESENTATION_MODE_FIFO => PresentMode::Fifo,
        }
    }
}

/// Represents a power preference.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDGLPowerPreference {
    NSTD_GL_POWER_PREFERENCE_DEFAULT,
    NSTD_GL_POWER_PREFERENCE_LOW,
    NSTD_GL_POWER_PREFERENCE_HIGH,
}
impl Into<PowerPreference> for NSTDGLPowerPreference {
    #[inline]
    fn into(self) -> PowerPreference {
        match self {
            Self::NSTD_GL_POWER_PREFERENCE_DEFAULT => PowerPreference::default(),
            Self::NSTD_GL_POWER_PREFERENCE_LOW => PowerPreference::LowPower,
            Self::NSTD_GL_POWER_PREFERENCE_HIGH => PowerPreference::HighPerformance,
        }
    }
}

/// Configures a GL state upon creation.
/// For `backend`, `NSTD_GL_BACKEND_UNKNOWN` will pick a default backend to use.
#[repr(C)]
pub struct NSTDGLStateDescriptor {
    pub backend: NSTDGLBackend,
    pub power_preference: NSTDGLPowerPreference,
    pub presentation_mode: NSTDGLPresentationMode,
}

/// Creates a new GL state.
/// Parameters:
///     `NSTDWindow window` - The window in which the GL state will live in.
///     `const NSTDGLStateDescriptor descriptor` - Configures the state.
/// Returns: `NSTDGLState state` - The new GL state.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_state_new(
    window: NSTDWindow,
    descriptor: NSTDGLStateDescriptor,
) -> NSTDGLState {
    let instance = Instance::new(descriptor.backend.into());
    let surface = instance.create_surface(&*window);
    let adapter = match executor::block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: descriptor.power_preference.into(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    })) {
        Some(adapter) => adapter,
        _ => return NSTDGLState::default(),
    };
    let (device, queue) =
        match executor::block_on(adapter.request_device(&DeviceDescriptor::default(), None)) {
            Ok((device, queue)) => (device, queue),
            _ => return NSTDGLState::default(),
        };
    let size = nstd_gui::nstd_std_gui_window_get_client_size(window);
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
    NSTDGLState {
        surface: Box::into_raw(Box::new(surface)),
        config: Box::into_raw(Box::new(config)),
        device_handle: Box::into_raw(Box::new(adapter)),
        device: Box::into_raw(Box::new(device)),
        queue: Box::into_raw(Box::new(queue)),
        size,
        clear_color: NSTDGLColor::default(),
    }
}

/// Pushes the current frame to the display.
/// Parameters:
///     `const NSTDGLState *const state` - The GL state.
///     `void(*callback)(NSTDGLRenderPass)` - Manipulates the render pass.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_state_render(
    state: &NSTDGLState,
    callback: extern "C" fn(NSTDGLRenderPass),
) -> c_int {
    let output = match (*state.surface).get_current_texture() {
        Ok(output) => output,
        _ => return 1,
    };
    let view = output
        .texture
        .create_view(&TextureViewDescriptor::default());
    let mut encoder =
        (*state.device).create_command_encoder(&CommandEncoderDescriptor { label: None });
    {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: state.clear_color.r,
                        g: state.clear_color.g,
                        b: state.clear_color.b,
                        a: state.clear_color.a,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        callback(&mut render_pass);
    }
    (*state.queue).submit(std::iter::once(encoder.finish()));
    output.present();
    0
}

/// Resizes a GL state's context.
/// Parameters:
///     `NSTDGLState *const state` - The GL state.
///     `const NSTDWindowSize *const new_size` - The new context size.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_state_resize(
    state: &mut NSTDGLState,
    new_size: &NSTDWindowSize,
) {
    if new_size.width > 0 && new_size.height > 0 {
        state.size = *new_size;
        (*state.config).width = new_size.width;
        (*state.config).height = new_size.height;
        (*state.surface).configure(&*state.device, &*state.config);
    }
}

/// Frees and destroys a GL state.
/// Parameters:
///     `NSTDGLState *const state` - The GL state.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_state_free(state: &mut NSTDGLState) {
    Box::from_raw(state.surface);
    Box::from_raw(state.config);
    Box::from_raw(state.device_handle);
    Box::from_raw(state.device);
    Box::from_raw(state.queue);
    state.surface = ptr::null_mut();
    state.config = ptr::null_mut();
    state.device_handle = ptr::null_mut();
    state.device = ptr::null_mut();
    state.queue = ptr::null_mut();
}

/// Retrieves info on a device.
/// Parameters:
///     `NSTDGLDeviceHandle device_handle` - Handle to a device.
/// Returns: `NSTDGLDeviceInfo device_info` - Contains information about a device.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_device_handle_get_info(
    device_handle: NSTDGLDeviceHandle,
) -> NSTDGLDeviceInfo {
    let info = (*device_handle).get_info();
    NSTDGLDeviceInfo {
        name: {
            let mut bytes = info.name.into_bytes();
            bytes.push(0);
            CString::from_vec_unchecked(bytes).into_raw()
        },
        vendor: info.vendor,
        device: info.device,
        device_type: match info.device_type {
            DeviceType::Other => NSTDGLDeviceType::NSTD_GL_DEVICE_TYPE_UNKNOWN,
            DeviceType::IntegratedGpu => NSTDGLDeviceType::NSTD_GL_DEVICE_TYPE_INTEGRATED_GPU,
            DeviceType::DiscreteGpu => NSTDGLDeviceType::NSTD_GL_DEVICE_TYPE_DISCRETE_GPU,
            DeviceType::VirtualGpu => NSTDGLDeviceType::NSTD_GL_DEVICE_TYPE_VIRTUAL_GPU,
            DeviceType::Cpu => NSTDGLDeviceType::NSTD_GL_DEVICE_TYPE_CPU,
        },
        backend: match info.backend {
            Backend::Empty => NSTDGLBackend::NSTD_GL_BACKEND_UNKNOWN,
            Backend::Vulkan => NSTDGLBackend::NSTD_GL_BACKEND_VULKAN,
            Backend::Metal => NSTDGLBackend::NSTD_GL_BACKEND_METAL,
            Backend::Dx12 => NSTDGLBackend::NSTD_GL_BACKEND_DX12,
            Backend::Dx11 => NSTDGLBackend::NSTD_GL_BACKEND_DX11,
            Backend::Gl => NSTDGLBackend::NSTD_GL_BACKEND_GL,
            Backend::BrowserWebGpu => NSTDGLBackend::NSTD_GL_BACKEND_WEBGPU,
        },
    }
}

/// Creates a new shader module.
/// Parameters:
///     `const NSTDByte *const data` - Raw spirv data.
///     `const NSTDSize size` - Number of bytes of spirv data.
///     `NSTDGLDevice device` - The device to create the shader module on.
/// Returns: `NSTDGLShaderModule shader` - The new shader module.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_shader_module_new(
    data: *const u8,
    size: usize,
    device: NSTDGLDevice,
) -> NSTDGLShaderModule {
    let data = std::slice::from_raw_parts(data, size);
    let source = ShaderSource::SpirV(wgpu::util::make_spirv_raw(data));
    let descriptor = ShaderModuleDescriptor {
        label: None,
        source,
    };
    Box::into_raw(Box::new((*device).create_shader_module(&descriptor)))
}

/// Frees a shader module.
/// Parameters:
///     `NSTDGLShaderModule *shader` - Pointer to a shader module.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_shader_module_free(shader: &mut NSTDGLShaderModule) {
    Box::from_raw(*shader);
    *shader = ptr::null_mut();
}

/// Creates a new render pipeline from a vertex and fragment shader.
/// Parameters:
///     `NSTDGLShaderModule vert` - The vertex shader module.
///     `NSTDGLShaderModule frag` - The fragment shader module.
///     `NSTDGLDevice device` - The device to create the render pipeline on.
///     `NSTDGLSurfaceConfiguration config` - The surface configuration.
/// Returns: `NSTDGLRenderPipeline pipeline` - The new render pipeline.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_render_pipeline_new(
    vert: NSTDGLShaderModule,
    frag: NSTDGLShaderModule,
    device: NSTDGLDevice,
    config: NSTDGLSurfaceConfiguration,
) -> NSTDGLRenderPipeline {
    let layout_descriptor = PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    };
    let layout = (*device).create_pipeline_layout(&layout_descriptor);
    Box::into_raw(Box::new((*device).create_render_pipeline(
        &RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: VertexState {
                module: &*vert,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &*frag,
                entry_point: "main",
                targets: &[ColorTargetState {
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                    format: (*config).format,
                }],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        },
    )))
}

/// Frees a render pipeline.
/// Parameters:
///     `NSTDGLRenderPipeline *pipeline` - Pointer to a render pipeline.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_render_pipeline_free(pipeline: &mut NSTDGLRenderPipeline) {
    Box::from_raw(*pipeline);
    *pipeline = ptr::null_mut();
}

/// Sets a render pipeline for a render pass.
/// Parameters:
///     `NSTDGLRenderPass render_pass` - The render pass.
///     `NSTDGLRenderPipeline pipeline` - The render pipeline.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_render_pass_set_pipeline(
    render_pass: NSTDGLRenderPass,
    pipeline: NSTDGLRenderPipeline,
) {
    (*render_pass).set_pipeline(&*pipeline);
}

/// Draws primitives from active vertex buffers.
/// Parameters:
///     `NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDUInt32 verticies` - Number of verticies to draw.
///     `const NSTDUInt32 instances` - Number of instnaces.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_render_pass_draw(
    render_pass: NSTDGLRenderPass,
    verticies: u32,
    instances: u32,
) {
    (*render_pass).draw(0..verticies, 0..instances);
}

/// Frees an `NSTDGLDeviceInfo` object.
/// Parameters:
///     `NSTDGLDeviceInfo *const device_info` - Pointer to an `NSTDGLDeviceInfo` object.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_device_info_free(device_info: &mut NSTDGLDeviceInfo) {
    CString::from_raw(device_info.name);
    device_info.name = ptr::null_mut();
}
