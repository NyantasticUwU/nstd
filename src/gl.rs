pub mod buffer;
pub mod def;
pub mod device;
pub mod pipeline;
pub mod state;
pub mod surface;
use self::{
    buffer::{NSTDGLBuffer, NSTDGLIndexFormat, NSTDGLVertexBufferLayout},
    def::NSTDGLBackend,
    device::{NSTDGLDevice, NSTDGLDeviceHandle, NSTDGLDeviceInfo, NSTDGLDeviceType},
    pipeline::{NSTDGLRenderPass, NSTDGLRenderPipeline, NSTDGLShaderModule},
    surface::NSTDGLSurfaceConfiguration,
};
use crate::{core::slice::NSTDSlice, string::NSTDString};
use wgpu::{util::*, *};

/// Retrieves info on a device.
/// Parameters:
///     `const NSTDGLDeviceHandle device_handle` - Handle to a device.
/// Returns: `NSTDGLDeviceInfo device_info` - Contains information about a device.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_device_handle_get_info(
    device_handle: NSTDGLDeviceHandle,
) -> NSTDGLDeviceInfo {
    let info = (*device_handle).get_info();
    NSTDGLDeviceInfo {
        name: NSTDString::from(info.name.as_bytes()),
        vendor: info.vendor,
        device: info.device,
        device_type: NSTDGLDeviceType::from(info.device_type),
        backend: NSTDGLBackend::from(info.backend),
    }
}

/// Creates a new shader module.
/// Parameters:
///     `const NSTDSlice *const data` - Raw spirv data.
///     `const NSTDGLDevice device` - The device to create the shader module on.
/// Returns: `NSTDGLShaderModule shader` - The new shader module.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_shader_module_new(
    data: &NSTDSlice,
    device: NSTDGLDevice,
) -> NSTDGLShaderModule {
    let data = std::slice::from_raw_parts(data.ptr.raw.cast(), data.size);
    let source = ShaderSource::SpirV(wgpu::util::make_spirv_raw(data));
    let descriptor = ShaderModuleDescriptor {
        label: None,
        source,
    };
    Box::into_raw(Box::new((*device).create_shader_module(&descriptor)))
}

/// Frees a shader module.
/// Parameters:
///     `NSTDGLShaderModule *const shader` - Pointer to a shader module.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_shader_module_free(shader: &mut NSTDGLShaderModule) {
    Box::from_raw(*shader);
    *shader = std::ptr::null_mut();
}

/// Creates a new render pipeline from a vertex and fragment shader.
/// Parameters:
///     `const NSTDGLShaderModule vert` - The vertex shader module.
///     `const NSTDGLShaderModule frag` - The fragment shader module.
///     `const NSTDSlice *const buffers` - A slice of `NSTDGLVertexBufferLayout`s.
///     `const NSTDGLDevice device` - The device to create the render pipeline on.
///     `const NSTDGLSurfaceConfiguration config` - The surface configuration.
/// Returns: `NSTDGLRenderPipeline pipeline` - The new render pipeline.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_render_pipeline_new(
    vert: NSTDGLShaderModule,
    frag: NSTDGLShaderModule,
    buffers: &NSTDSlice,
    device: NSTDGLDevice,
    config: NSTDGLSurfaceConfiguration,
) -> NSTDGLRenderPipeline {
    let layout_descriptor = PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    };
    let layout = (*device).create_pipeline_layout(&layout_descriptor);
    let data = std::slice::from_raw_parts(
        buffers.ptr.raw as *const NSTDGLVertexBufferLayout,
        buffers.size,
    );
    let mut buffers = Vec::<VertexBufferLayout>::new();
    buffers.try_reserve(data.len()).ok();
    for buffer in data {
        buffers.push((*buffer).into());
    }
    Box::into_raw(Box::new((*device).create_render_pipeline(
        &RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: VertexState {
                module: &*vert,
                entry_point: "main",
                buffers: &buffers,
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
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        },
    )))
}

/// Frees a render pipeline.
/// Parameters:
///     `NSTDGLRenderPipeline *const pipeline` - Pointer to a render pipeline.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_render_pipeline_free(pipeline: &mut NSTDGLRenderPipeline) {
    Box::from_raw(*pipeline);
    *pipeline = std::ptr::null_mut();
}

/// Sets a render pipeline for a render pass.
/// Parameters:
///     `const NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDGLRenderPipeline pipeline` - The render pipeline.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_render_pass_set_pipeline(
    render_pass: NSTDGLRenderPass,
    pipeline: NSTDGLRenderPipeline,
) {
    (*render_pass).set_pipeline(&*pipeline);
}

/// Sets a render pass' vertex buffer.
/// Parameters:
///     `const NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDGLBuffer buffer` - The GPU vertex buffer.
///     `const NSTDUInt32 slot` - The buffer slot (the index of the buffer layout).
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_render_pass_set_vertex_buffer(
    render_pass: NSTDGLRenderPass,
    buffer: NSTDGLBuffer,
    slot: u32,
) {
    (*render_pass).set_vertex_buffer(slot, (*buffer).slice(..));
}

/// Sets a render pass' index buffer.
/// Parameters:
///     `const NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDGLBuffer buffer` - The GPU index buffer.
///     `const NSTDGLIndexFormat format` - The index format of the buffer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_render_pass_set_index_buffer(
    render_pass: NSTDGLRenderPass,
    buffer: NSTDGLBuffer,
    format: NSTDGLIndexFormat,
) {
    (*render_pass).set_index_buffer((*buffer).slice(..), format.into());
}

/// Draws primitives from active vertex buffers.
/// Parameters:
///     `const NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDUInt32 verticies` - Number of verticies to draw.
///     `const NSTDUInt32 instances` - Number of instnaces.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_render_pass_draw(
    render_pass: NSTDGLRenderPass,
    verticies: u32,
    instances: u32,
) {
    (*render_pass).draw(0..verticies, 0..instances);
}

/// Draws primitives from active vertex and index buffers.
/// Parameters:
///     `const NSTDGLRenderPass render_pass` - The render pass.
///     `const NSTDUInt32 indicies` - The indicies to draw.
///     `const NSTDUInt32 instances` - The instances to draw.
///     `const NSTDInt32 base` - The base vertex.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_render_pass_draw_indexed(
    render_pass: NSTDGLRenderPass,
    indicies: u32,
    instances: u32,
    base: i32,
) {
    (*render_pass).draw_indexed(0..indicies, base, 0..instances);
}

/// Frees an `NSTDGLDeviceInfo` object.
/// Parameters:
///     `NSTDGLDeviceInfo *const device_info` - Pointer to an `NSTDGLDeviceInfo` object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_device_info_free(device_info: &mut NSTDGLDeviceInfo) {
    crate::string::nstd_string_free(&mut device_info.name);
}

/// Creates a new GPU buffer.
/// Parameters:
///     `const NSTDSlice *const bytes` - The bytes to send to the GPU.
///     `const NSTDGLDevice device` - The device to create the buffer on.
/// Returns: `NSTDGLBuffer buffer` - The new GPU buffer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_buffer_new(
    bytes: &NSTDSlice,
    device: NSTDGLDevice,
) -> NSTDGLBuffer {
    Box::into_raw(Box::new((*device).create_buffer_init(
        &BufferInitDescriptor {
            label: None,
            contents: std::slice::from_raw_parts(bytes.ptr.raw.cast(), bytes.byte_count()),
            usage: BufferUsages::all(),
        },
    )))
}

/// Frees a GPU buffer.
/// Parameters:
///     `NSTDGLBuffer *const buffer` - The GPU buffer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_buffer_free(buffer: &mut NSTDGLBuffer) {
    Box::from_raw(*buffer);
    *buffer = std::ptr::null_mut();
}
