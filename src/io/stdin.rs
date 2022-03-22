//! The standard input stream.
use super::{input_stream::NSTDInputStream, stream::NSTDStream};
use crate::{core::def::NSTDAny, string::NSTDString, vec::NSTDVec};
use std::io::{prelude::*, BufReader, Stdin};

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

/// Frees a handle to stdin.
/// Parameters:
///     `NSTDStandardInput *const stdin` - The standard input stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdin_free(stdin: &mut NSTDStandardInput) {
    Box::from_raw(stdin.handle);
    stdin.handle = std::ptr::null_mut();
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
