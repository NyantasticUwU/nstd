//! Interaction with the display's surface.
pub mod config;
pub mod texture;
use crate::{gl::instance::NSTDGLInstance, gui::window::NSTDWindow};
use wgpu::Surface;

/// Represents a graphical surface.
pub type NSTDGLSurface = *mut Surface;

/// Creates a new surface.
/// Parameters:
///     `const NSTDGLInstance instance` - The instance to create the surface with.
///     `const NSTDWindow window` - The window to get the surface from.
/// Returns: `NSTDGLSurface surface` - The surface of the window.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_surface_new(
    instance: NSTDGLInstance,
    window: NSTDWindow,
) -> NSTDGLSurface {
    Box::into_raw(Box::new((*instance).create_surface(&*window)))
}

/// Frees a surface.
/// Parameters:
///     `NSTDGLSurface *const surface` - The surface to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_surface_free(surface: &mut NSTDGLSurface) {
    Box::from_raw(*surface);
    *surface = std::ptr::null_mut();
}
