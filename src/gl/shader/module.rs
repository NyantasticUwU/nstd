//! GPU shader programs.
use crate::{core::slice::NSTDSlice, gl::device::NSTDGLDevice};
use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};

/// Represents a shader module.
pub type NSTDGLShaderModule = *mut ShaderModule;

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
    Box::into_raw(Box::new((*device.raw).create_shader_module(&descriptor)))
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
