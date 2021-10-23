use futures::executor;
use nstd_gui::{NSTDWindow, NSTDWindowSize};
use std::{
    os::raw::{c_double, c_int},
    ptr,
};
use wgpu::{
    Backends, BlendState, Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor, Device,
    DeviceDescriptor, Face, FragmentState, FrontFace, Instance, LoadOp, MultisampleState,
    Operations, PipelineLayoutDescriptor, PolygonMode, PowerPreference, PresentMode,
    PrimitiveState, PrimitiveTopology, Queue, RenderPass, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions,
    ShaderModule, ShaderModuleDescriptor, ShaderSource, Surface, SurfaceConfiguration,
    TextureUsages, TextureViewDescriptor, VertexState,
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
    pub device: NSTDGLDevice,
    pub queue: NSTDGLQueue,
    pub size: NSTDWindowSize,
    pub clear_color: NSTDGLColor,
}
impl Default for NSTDGLState {
    fn default() -> Self {
        Self {
            surface: ptr::null_mut(),
            device: ptr::null_mut(),
            queue: ptr::null_mut(),
            config: ptr::null_mut(),
            ..Default::default()
        }
    }
}

/// Creates a new GL state.
/// Parameters:
///     `NSTDWindow window` - The window in which the GL state will live in.
/// Returns: `NSTDGLState state` - The new GL state.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_gl_state_new(window: NSTDWindow) -> NSTDGLState {
    let instance = Instance::new(Backends::all());
    let surface = instance.create_surface(&*window);
    let adapter = match executor::block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::default(),
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
        present_mode: PresentMode::Fifo,
    };
    surface.configure(&device, &config);
    NSTDGLState {
        surface: Box::into_raw(Box::new(surface)),
        device: Box::into_raw(Box::new(device)),
        queue: Box::into_raw(Box::new(queue)),
        config: Box::into_raw(Box::new(config)),
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
    Box::from_raw(state.device);
    Box::from_raw(state.queue);
    Box::from_raw(state.config);
    state.surface = ptr::null_mut();
    state.device = ptr::null_mut();
    state.queue = ptr::null_mut();
    state.config = ptr::null_mut();
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
