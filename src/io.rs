pub mod input_stream;
pub mod io_stream;
pub mod output_stream;
pub mod stderr;
pub mod stdin;
pub mod stdout;
pub mod stream;
use self::{
    input_stream::{NSTDInputStream, NSTDRawInputStream},
    output_stream::{NSTDOutputStream, NSTDRawOutputStream},
    stderr::NSTDStandardError,
    stdin::NSTDStandardInput,
    stdout::NSTDStandardOutput,
    stream::NSTDStream,
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
        stream: NSTDStream::default(),
        istream: NSTDRawInputStream::new(Box::new(BufReader::new(std::io::stdin()))),
        read: Some(istream_read),
        read_line: Some(istream_read_line),
    }
}

/// Returns a handle to stdout.
/// Returns: `NSTDStandardOutput stdout` - The standard output stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdout() -> NSTDStandardOutput {
    NSTDStandardOutput {
        stream: NSTDStream::default(),
        ostream: NSTDRawOutputStream::new(Box::new(std::io::stdout())),
        flush: Some(ostream_flush),
        write: Some(ostream_write),
    }
}

/// Returns a handle to stderr.
/// Returns: `NSTDStandardError stderr` - The standard error stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stderr() -> NSTDStandardError {
    NSTDStandardError {
        stream: NSTDStream::default(),
        ostream: NSTDRawOutputStream::new(Box::new(std::io::stderr())),
        flush: Some(ostream_flush),
        write: Some(ostream_write),
    }
}

/// Reads contents of `NSTDInputStream` into a vector.
#[inline]
unsafe extern "C" fn istream_read(this: NSTDAny) -> NSTDVec {
    let this = this as *mut NSTDInputStream;
    let mut bytes = Vec::new();
    if (*this).istream.read_to_end(&mut bytes).is_err() {
        (*this).stream.errc = 1;
    }
    NSTDVec::from(bytes)
}

/// Reads a string from `NSTDInputStream` into a string.
#[inline]
unsafe extern "C" fn istream_read_line(this: NSTDAny) -> NSTDString {
    let this = this as *mut NSTDInputStream;
    let mut string = String::new();
    if (*this).istream.read_line(&mut string).is_err() {
        (*this).stream.errc = 1;
    }
    NSTDString::from(string.into_bytes())
}

/// Flushes an `NSTDOutputStream`.
#[inline]
unsafe extern "C" fn ostream_flush(this: NSTDAny) {
    let this = this as *mut NSTDOutputStream;
    if (*this).ostream.flush().is_err() {
        (*this).stream.errc = 1;
    }
}

/// Writes a buffer to an `NSTDOutputStream`.
#[inline]
unsafe extern "C" fn ostream_write(this: NSTDAny, buffer: &NSTDSlice) {
    let this = this as *mut NSTDOutputStream;
    let buffer = std::slice::from_raw_parts(buffer.ptr.raw.cast(), buffer.size);
    if (*this).ostream.write_all(buffer).is_err() {
        (*this).stream.errc = 1;
    }
}
