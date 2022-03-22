//! The surface's configuration.
use crate::{
    gl::{def::NSTDGLPresentationMode, texture::NSTDGLTextureFormat},
    gui::def::NSTDWindowSize,
};
use wgpu::{SurfaceConfiguration, TextureUsages};

/// Represents a surface config.
pub type NSTDGLSurfaceConfig = *mut SurfaceConfiguration;

/// Creates a new surface configuration.
///
/// # Parameters
///
/// - `const NSTDWindowSize size` - The size of the viewport should be.
///
/// - `const NSTDGLTextureFormat texture_format` - The texture format to use.
///
/// - `const NSTDGLPresentationMode presentation_mode` - The presentation mode to use.
///
/// # Returns
///
/// `NSTDGLSurfaceConfig config` - The surface configuration.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_surface_config_new(
    size: NSTDWindowSize,
    texture_format: NSTDGLTextureFormat,
    presentation_mode: NSTDGLPresentationMode,
) -> NSTDGLSurfaceConfig {
    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: *texture_format,
        width: size.width,
        height: size.height,
        present_mode: presentation_mode.into(),
    };
    Box::into_raw(Box::new(config))
}

/// Frees a surface configuration.
///
/// # Parameters
///
/// - `NSTDGLSurfaceConfiguration *const config` - The surface configuration to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_surface_config_free(config: &mut NSTDGLSurfaceConfig) {
    Box::from_raw(*config);
    *config = std::ptr::null_mut();
}
