//! Surface textures.
use super::NSTDGLSurface;
use wgpu::SurfaceTexture;

/// A surface texture.
pub type NSTDGLSurfaceTexture = *mut SurfaceTexture;

/// Gets a surface's current texture.
///
/// # Parameters
///
/// - `const NSTDGLSurface surface` - The surface.
///
/// # Returns
///
/// `NSTDGLSurfaceTexture surface_texture` - The surface's texture.
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

/// Presents a surface texture to the surface.
///
/// # Note
///
/// This function will free `surface_texture`.
///
/// # Parameters
///
/// - `NSTDGLSurfaceTexture *const surface_texture` - The surface texture to present.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_surface_texture_present(
    surface_texture: &mut NSTDGLSurfaceTexture,
) {
    let texture = Box::from_raw(*surface_texture);
    *surface_texture = std::ptr::null_mut();
    texture.present();
}

/// Frees a surface texture.
///
/// # Parameters
///
/// - `NSTDGLSurfaceTexture *const surface_texture` - The surface texture to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_surface_texture_free(surface_texture: &mut NSTDGLSurfaceTexture) {
    Box::from_raw(*surface_texture);
    *surface_texture = std::ptr::null_mut();
}
