use crate::gl::surface::texture::NSTDGLSurfaceTexture;
use wgpu::{TextureView, TextureViewDescriptor};

/// Represents a view of a texture.
pub type NSTDGLTextureView = *mut TextureView;

/// Creates a new texture view.
/// Parameters:
///     `const NSTDGLSurfaceTexture surface_texture` - A surface texture.
/// Returns: `NSTDGLTextureView texture_view` - The new texture view.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_texture_view_from_surface_texture(
    surface_texture: NSTDGLSurfaceTexture,
) -> NSTDGLTextureView {
    let view_desc = TextureViewDescriptor::default();
    Box::into_raw(Box::new((*surface_texture).texture.create_view(&view_desc)))
}

/// Frees a texture view.
/// Parameters:
///     `NSTDGLTextureView *const texture_view` - The texture view to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_texture_view_free(texture_view: &mut NSTDGLTextureView) {
    Box::from_raw(*texture_view);
    *texture_view = std::ptr::null_mut();
}
