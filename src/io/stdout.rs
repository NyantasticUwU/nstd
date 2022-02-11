use crate::io::output_stream::NSTDOutputStream;

/// Represents a handle to the standard output stream.
pub type NSTDStandardOutput = NSTDOutputStream;

/// Frees a handle to stdout.
/// Parameters:
///     `NSTDStandardOutput stdout` - The standard output stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdout_free(stdout: NSTDStandardOutput) {
    drop(stdout.ostream);
}
