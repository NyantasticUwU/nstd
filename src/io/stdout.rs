use crate::io::output_stream::NSTDOutputStream;
use std::io::Stdout;

/// A raw handle to stdout.
pub type NSTDStandardOutputHandle = Box<Stdout>;

/// Represents a handle to the standard output stream.
#[repr(C)]
pub struct NSTDStandardOutput {
    /// The output stream.
    pub output_stream: NSTDOutputStream,
    /// The raw handle to stdout.
    pub handle: NSTDStandardOutputHandle,
}

/// Frees a handle to stdout.
/// Parameters:
///     `NSTDStandardOutput stdout` - The standard output stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdout_free(stdout: NSTDStandardOutput) {
    drop(stdout.handle);
}
