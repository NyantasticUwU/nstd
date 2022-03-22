//! The standard output stream.
use super::{output_stream::NSTDOutputStream, stream::NSTDStream};
use crate::core::{def::NSTDAny, slice::NSTDSlice};
use std::io::{prelude::*, Stdout};

/// A raw handle to stdout.
pub type NSTDStandardOutputHandle = *mut Stdout;

/// Represents a handle to the standard output stream.
#[repr(C)]
pub struct NSTDStandardOutput {
    /// The output stream.
    pub output_stream: NSTDOutputStream,
    /// The raw handle to stdout.
    pub handle: NSTDStandardOutputHandle,
}

/// Returns a handle to stdout.
/// Returns: `NSTDStandardOutput stdout` - The standard output stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdout() -> NSTDStandardOutput {
    NSTDStandardOutput {
        output_stream: NSTDOutputStream {
            stream: NSTDStream::default(),
            flush: Some(stdout_flush),
            write: Some(stdout_write),
        },
        handle: Box::into_raw(Box::new(std::io::stdout())),
    }
}

/// Frees a handle to stdout.
/// Parameters:
///     `NSTDStandardOutput *const stdout` - The standard output stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdout_free(stdout: &mut NSTDStandardOutput) {
    Box::from_raw(stdout.handle);
    stdout.handle = std::ptr::null_mut();
}

/// Flushes an `NSTDStandardOutput`.
#[inline]
unsafe extern "C" fn stdout_flush(this: NSTDAny) {
    let this = this as *mut NSTDStandardOutput;
    if (*(*this).handle).flush().is_err() {
        (*this).output_stream.stream.errc = 1;
    }
}

/// Writes a buffer to an `NSTDStandardOutput`.
#[inline]
unsafe extern "C" fn stdout_write(this: NSTDAny, buffer: &NSTDSlice) {
    let this = this as *mut NSTDStandardOutput;
    let buffer = std::slice::from_raw_parts(buffer.ptr.raw.cast(), buffer.size);
    if (*(*this).handle).write_all(buffer).is_err() {
        (*this).output_stream.stream.errc = 1;
    }
}
