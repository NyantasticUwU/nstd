//! The standard error stream.
use super::{output_stream::NSTDOutputStream, stream::NSTDStream};
use crate::core::{def::NSTDAny, slice::NSTDSlice};
use std::io::{prelude::*, Stderr};

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

/// Returns a handle to stderr.
///
/// # Returns
///
/// `NSTDStandardError stderr` - The standard error stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stderr() -> NSTDStandardError {
    NSTDStandardError {
        output_stream: NSTDOutputStream {
            stream: NSTDStream::default(),
            flush: Some(stderr_flush),
            write: Some(stderr_write),
        },
        handle: Box::into_raw(Box::new(std::io::stderr())),
    }
}

/// Frees a handle to stderr.
///
/// # Parameters
///
/// - `NSTDStandardError *const stderr` - The standard error stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stderr_free(stderr: &mut NSTDStandardError) {
    Box::from_raw(stderr.handle);
    stderr.handle = std::ptr::null_mut();
}

/// Flushes an `NSTDStandardError`.
#[inline]
unsafe extern "C" fn stderr_flush(this: NSTDAny) {
    let this = this as *mut NSTDStandardError;
    if (*(*this).handle).flush().is_err() {
        (*this).output_stream.stream.errc = 1;
    }
}

/// Writes a buffer to an `NSTDStandardError`.
#[inline]
unsafe extern "C" fn stderr_write(this: NSTDAny, buffer: &NSTDSlice) {
    let this = this as *mut NSTDStandardError;
    let buffer = std::slice::from_raw_parts(buffer.ptr.raw.cast(), buffer.size);
    if (*(*this).handle).write_all(buffer).is_err() {
        (*this).output_stream.stream.errc = 1;
    }
}
