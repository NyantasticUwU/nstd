use wgpu::{RenderPass, RenderPipeline, ShaderModule};

/// Represents a shader module.
pub type NSTDGLShaderModule = *mut ShaderModule;

/// Represents a render pipeline.
pub type NSTDGLRenderPipeline = *mut RenderPipeline;

/// Represents a render pass object.
pub type NSTDGLRenderPass<'a> = *mut RenderPass<'a>;
