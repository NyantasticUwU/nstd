pub mod view;
use crate::gl::{device::handle::NSTDGLDeviceHandle, surface::NSTDGLSurface};
use wgpu::{SurfaceTexture, TextureFormat};

/// A surface texture.
pub type NSTDGLSurfaceTexture = *mut SurfaceTexture;

/// A texture format.
pub type NSTDGLTextureFormat = *mut TextureFormat;

/// Gets a surface's current texture.
/// Parameters:
///     `const NSTDGLSurface surface` - The surface.
/// Returns: `NSTDGLSurfaceTexture surface_texture` - The surface's texture.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_surface_texture_current(
    surface: NSTDGLSurface,
) -> NSTDGLSurfaceTexture {
    match (*surface).get_current_texture() {
        Ok(surface_texture) => Box::into_raw(Box::new(surface_texture)),
        _ => std::ptr::null_mut(),
    }
}

/// Frees a surface texture.
/// Parameters:
///     `NSTDGLSurfaceTexture surface_texture` - The surface texture to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_surface_texture_free(surface_texture: &mut NSTDGLSurfaceTexture) {
    Box::from_raw(*surface_texture);
    *surface_texture = std::ptr::null_mut();
}

/// Gets the surface's preferred format.
/// Parameters:
///     `const NSTDGLSurface surface` - The surface.
///     `const NSTDGLDeviceHandle device_handle` - The device handle.
/// Returns: `NSTDGLTextureFormat texture_format` - The preferred texture format.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_texture_format_default(
    surface: NSTDGLSurface,
    device_handle: NSTDGLDeviceHandle,
) -> NSTDGLTextureFormat {
    match (*surface).get_preferred_format(&*device_handle) {
        Some(texture_format) => Box::into_raw(Box::new(texture_format)),
        _ => std::ptr::null_mut(),
    }
}

/// Frees an `NSTDGLTextureFormat`.
/// Parameters:
///     `NSTDGLTextureFormat *const texture_format` - The texture format to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_texture_format_free(texture_format: &mut NSTDGLTextureFormat) {
    Box::from_raw(*texture_format);
    *texture_format = std::ptr::null_mut();
}
