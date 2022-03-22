//! Standard I/O.
pub mod input_stream;
pub mod io_stream;
pub mod output_stream;
pub mod stderr;
pub mod stdin;
pub mod stdout;
pub mod stream;
use crate::{
    core::def::{NSTDChar, NSTDErrorCode},
    string::NSTDString,
};
use std::io::{prelude::*, BufReader};

/// Writes a C string to stdout.
///
/// # Parameters
///
/// - `const NSTDChar *const msg` - The message to write to stdout.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_print(msg: *const NSTDChar) -> NSTDErrorCode {
    let size = crate::core::cstr::nstd_core_cstr_len(msg);
    let buffer = std::slice::from_raw_parts(msg.cast(), size);
    match std::io::stdout().write_all(buffer) {
        Ok(_) => 0,
        _ => 1,
    }
}

/// Writes a C string to stdout with a preceding new line.
///
/// # Parameters
///
/// - `const NSTDChar *const msg` - The message to write to stdout.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_print_line(msg: *const NSTDChar) -> NSTDErrorCode {
    nstd_io_print(msg) | nstd_io_print(b"\n\0".as_ptr().cast())
}

/// Reads a line from stdin as an `NSTDString` but doesn't include the new line.
///
/// # Returns
///
/// `NSTDString input` - Input read from stdin.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_read() -> NSTDString {
    let mut input = nstd_io_read_line();
    if !input.bytes.buffer.ptr.raw.is_null() {
        let zero = crate::vec::nstd_vec_last(&input.bytes) as *mut u8;
        if !zero.is_null() {
            let nl = zero.sub(1);
            *nl = 0;
            crate::vec::nstd_vec_pop(&mut input.bytes);
        }
    }
    input
}

/// Reads a line from stdin as an `NSTDString`.
///
/// # Returns
///
/// `NSTDString input` - Input read from stdin.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_read_line() -> NSTDString {
    let mut buf = String::new();
    match BufReader::new(std::io::stdin()).read_line(&mut buf) {
        Ok(_) => {
            let mut buf = buf.into_bytes();
            buf.push(0);
            NSTDString::from(buf.as_slice())
        }
        _ => NSTDString::default(),
    }
}
