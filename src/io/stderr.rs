use crate::io::output_stream::NSTDOutputStream;
use std::io::Stderr;

/// A raw handle to stderr.
pub type NSTDStandardErrorHandle = *mut Stderr;

/// Represents a handle to the standard error stream.
#[repr(C)]
pub struct NSTDStandardError {
    /// The output stream.
    pub output_stream: NSTDOutputStream,
    /// The raw handle to stderr.
    pub handle: NSTDStandardErrorHandle,
}

/// Frees a handle to stderr.
/// Parameters:
///     `NSTDStandardError *const stderr` - The standard error stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stderr_free(stderr: &mut NSTDStandardError) {
    Box::from_raw(stderr.handle);
    stderr.handle = std::ptr::null_mut();
}
