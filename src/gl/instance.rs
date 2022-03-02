use wgpu::{Backends, Instance};

/// An instance of `nstd.gl`'s backend.
pub type NSTDGLInstance = *mut Instance;

/// Creates a new instance of `nstd.gl`'s backend `wgpu`.
/// Returns: `NSTDGLInstance instance` - The `wgpu` instance.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_instance_new() -> NSTDGLInstance {
    Box::into_raw(Box::new(Instance::new(Backends::all())))
}

/// Frees an instance of `nstd.gl`'s backend.
/// Parameters:
///     `NSTDGLInstance *const instance` The instance to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_gl_instance_free(instance: &mut NSTDGLInstance) {
    Box::from_raw(*instance);
    *instance = std::ptr::null_mut();
}
