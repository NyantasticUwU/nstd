use crate::io::output_stream::NSTDOutputStream;

/// Represents a handle to the standard error stream.
pub type NSTDStandardError = NSTDOutputStream;

/// Frees a handle to stderr.
/// Parameters:
///     `NSTDStandardError stderr` - The standard error stream.
pub unsafe extern "C" fn nstd_io_stdin_free(stdin: NSTDStandardError) {
    drop(stdin.ostream);
}
