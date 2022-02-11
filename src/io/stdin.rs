use crate::io::input_stream::NSTDInputStream;

/// Represents a handle to the standard input stream.
pub type NSTDStandardInput = NSTDInputStream;

/// Frees a handle to stdin.
/// Parameters:
///     `NSTDStandardInput stdin` - The standard input stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdin_free(stdin: NSTDStandardInput) {
    drop(stdin.istream);
}
