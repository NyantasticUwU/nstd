pub mod input_stream;
pub mod io_stream;
pub mod output_stream;
pub mod stderr;
pub mod stdin;
pub mod stdout;
pub mod stream;
use self::{
    input_stream::NSTDInputStream, output_stream::NSTDOutputStream, stderr::NSTDStandardError,
    stdin::NSTDStandardInput, stdout::NSTDStandardOutput, stream::NSTDStream,
};
use crate::{
    collections::vec::NSTDVec,
    core::{def::NSTDAny, slice::NSTDSlice},
    string::NSTDString,
};
use std::io::{prelude::*, BufReader};

/// Returns a handle to stdin.
/// Returns: `NSTDStandardInput stdin` - The standard input stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdin() -> NSTDStandardInput {
    NSTDStandardInput {
        input_stream: NSTDInputStream {
            stream: NSTDStream::default(),
            read: Some(stdin_read),
            read_exact: Some(stdin_read_exact),
            read_until: Some(stdin_read_until),
            read_line: Some(stdin_read_line),
        },
        handle: Box::into_raw(Box::new(BufReader::new(std::io::stdin()))),
    }
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

/// Returns a handle to stderr.
/// Returns: `NSTDStandardError stderr` - The standard error stream.
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

/// Reads contents of `NSTDStandardInput` into a vector.
#[inline]
unsafe extern "C" fn stdin_read(this: NSTDAny) -> NSTDVec {
    let this = this as *mut NSTDStandardInput;
    let mut bytes = Vec::new();
    if (*(*this).handle).read_to_end(&mut bytes).is_err() {
        (*this).input_stream.stream.errc = 1;
    }
    NSTDVec::from(bytes.as_slice())
}

/// Gets a specific number of bytes from stdin.
unsafe extern "C" fn stdin_read_exact(this: NSTDAny, count: usize) -> NSTDVec {
    let this = this as *mut NSTDStandardInput;
    let mut bytes = Vec::new();
    bytes.resize(count, 0);
    if (*(*this).handle).read_exact(&mut bytes).is_err() {
        (*this).input_stream.stream.errc = 1;
    }
    NSTDVec::from(bytes.as_slice())
}

/// Reads from stdin until `delimiter` is reached.
#[inline]
unsafe extern "C" fn stdin_read_until(this: NSTDAny, delimiter: u8) -> NSTDVec {
    let this = this as *mut NSTDStandardInput;
    let mut bytes = Vec::new();
    if (*(*this).handle).read_until(delimiter, &mut bytes).is_err() {
        (*this).input_stream.stream.errc = 1;
    }
    NSTDVec::from(bytes.as_slice())
}

/// Reads a string from `NSTDStandardInput` into a string.
#[inline]
unsafe extern "C" fn stdin_read_line(this: NSTDAny) -> NSTDString {
    let this = this as *mut NSTDStandardInput;
    let mut string = String::new();
    if (*(*this).handle).read_line(&mut string).is_err() {
        (*this).input_stream.stream.errc = 1;
    }
    NSTDString::from(string.as_bytes())
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
