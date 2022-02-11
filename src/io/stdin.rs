use crate::io::input_stream::NSTDInputStream;
use std::io::{BufReader, Stdin};

/// A raw handle to stdin.
pub type NSTDStandardInputHandle = Box<BufReader<Stdin>>;

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
///     `NSTDStandardInput stdin` - The standard input stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdin_free(stdin: NSTDStandardInput) {
    drop(stdin.handle);
}
