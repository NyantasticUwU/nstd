use crate::{
    core::slice::NSTDSlice,
    gl::{
        buffer::NSTDGLVertexBufferLayout, device::NSTDGLDevice, shader::module::NSTDGLShaderModule,
        surface::config::NSTDGLSurfaceConfig,
    },
};
use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Face, FragmentState, FrontFace, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline,
    RenderPipelineDescriptor, VertexBufferLayout, VertexState,
};

/// Represents a render pipeline.
pub type NSTDGLRenderPipeline = *mut RenderPipeline;

/// Creates a new render pipeline from a vertex and fragment shader.
/// Parameters:
///     `const NSTDGLShaderModule vert` - The vertex shader module.
///     `const NSTDGLShaderModule frag` - The fragment shader module.
///     `const NSTDSlice *const buffers` - A slice of `NSTDGLVertexBufferLayout`s.
///     `const NSTDGLDevice device` - The device to create the render pipeline on.
///     `const NSTDGLSurfaceConfig config` - The surface configuration.
/// Returns: `NSTDGLRenderPipeline pipeline` - The new render pipeline.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_render_pipeline_new(
    vert: NSTDGLShaderModule,
    frag: NSTDGLShaderModule,
    buffers: &NSTDSlice,
    device: NSTDGLDevice,
    config: NSTDGLSurfaceConfig,
) -> NSTDGLRenderPipeline {
    // Creating the pipeline layout.
    let layout_descriptor = PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    };
    let layout = (*device.raw).create_pipeline_layout(&layout_descriptor);
    // Getting the vertex buffer layouts.
    let buffers_ptr = buffers.ptr.raw as *const NSTDGLVertexBufferLayout;
    let data = std::slice::from_raw_parts(buffers_ptr, buffers.size);
    let mut buffers = Vec::<VertexBufferLayout>::new();
    buffers.try_reserve(data.len()).ok();
    for buffer in data {
        buffers.push((*buffer).into());
    }
    // Creating the pipeline.
    let color_target_state = ColorTargetState {
        blend: Some(BlendState::REPLACE),
        write_mask: ColorWrites::ALL,
        format: (*config).format,
    };
    let pipeline_desc = RenderPipelineDescriptor {
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
            targets: std::slice::from_ref(&color_target_state),
        }),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            polygon_mode: PolygonMode::Fill,
            strip_index_format: None,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    };
    let pipeline = (*device.raw).create_render_pipeline(&pipeline_desc);
    Box::into_raw(Box::new(pipeline))
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
