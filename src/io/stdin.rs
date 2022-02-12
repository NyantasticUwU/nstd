use crate::io::input_stream::NSTDInputStream;
use std::io::{BufReader, Stdin};

/// A raw handle to stdin.
pub type NSTDStandardInputHandle = *mut BufReader<Stdin>;

/// Represents a handle to the standard input stream.
#[repr(C)]
pub struct NSTDStandardInput {
    /// The input stream.
    pub input_stream: NSTDInputStream,
    /// The raw handle to stdin.
    pub handle: NSTDStandardInputHandle,
}

/// Frees a handle to stdin.
/// Parameters:
///     `NSTDStandardInput *const stdin` - The standard input stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdin_free(stdin: &mut NSTDStandardInput) {
    Box::from_raw(stdin.handle);
    stdin.handle = std::ptr::null_mut();
}
