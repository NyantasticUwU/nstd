//! Contains functions for working with a render pass.
use super::pipeline::NSTDGLRenderPipeline;
use crate::gl::{
    buffer::{NSTDGLBuffer, NSTDGLIndexFormat},
    command::encoder::NSTDGLCommandEncoder,
    def::NSTDGLColor,
    texture::view::NSTDGLTextureView,
};
use wgpu::{LoadOp, Operations, RenderPass, RenderPassColorAttachment, RenderPassDescriptor};

/// Represents a render pass object.
pub type NSTDGLRenderPass<'a> = *mut RenderPass<'a>;

/// Creates a new render pass.
/// Parameters:
///     `const NSTDGLCommandEncoder command_encoder` - The command encoder.
///     `const NSTDGLTextureView texture_view` - The texture view to render.
///     `const NSTDGLColor *const clear_color` - The clear color.
/// Returns: `NSTDGLRenderPass render_pass` - The new render pass.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_render_pass_new(
    command_encoder: NSTDGLCommandEncoder,
    texture_view: NSTDGLTextureView,
    clear_color: &NSTDGLColor,
) -> NSTDGLRenderPass {
    let render_pass_descriptor = RenderPassDescriptor {
        label: None,
        color_attachments: &[RenderPassColorAttachment {
            view: &*texture_view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(*clear_color),
                store: true,
            },
        }],
        depth_stencil_attachment: None,
    };
    let render_pass = (*command_encoder).begin_render_pass(&render_pass_descriptor);
    Box::into_raw(Box::new(render_pass))
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

/// Frees a render pass.
/// Parameters:
///     `NSTDGLRenderPass *const render_pass` - The render pass to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_render_pass_free(render_pass: &mut NSTDGLRenderPass) {
    Box::from_raw(*render_pass);
    *render_pass = std::ptr::null_mut();
}
