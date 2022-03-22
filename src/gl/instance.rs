//! An instance of `wgpu`.
use super::def::NSTDGLBackend;
use wgpu::Instance;

/// An instance of `nstd.gl`'s backend.
pub type NSTDGLInstance = *mut Instance;

/// Creates a new instance of `nstd.gl`'s backend `wgpu`.
///
/// # Parameters
///
/// - `const NSTDGLBackend backend` - The backend to use.
///
/// # Returns
///
/// `NSTDGLInstance instance` - The `wgpu` instance.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_instance_new(backend: NSTDGLBackend) -> NSTDGLInstance {
    Box::into_raw(Box::new(Instance::new(backend.into())))
}

/// Frees an instance of `nstd.gl`'s backend.
///
/// # Parameters
///
/// - `NSTDGLInstance *const instance` The instance to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_instance_free(instance: &mut NSTDGLInstance) {
    Box::from_raw(*instance);
    *instance = std::ptr::null_mut();
}
