use std::{
    ffi::{CStr, CString},
    io::{self, prelude::*, BufReader, BufWriter},
    os::raw::{c_char, c_int},
    ptr,
};

/// Attempts to flush stdout.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_io_flush() -> c_int {
    match io::stdout().flush() {
        Ok(_) => 0,
        _ => 1,
    }
}

/// Writes a single character to stdout.
/// Parameters:
///     `const char ch` - Character to write.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_io_write_char(ch: c_char) -> c_int {
    static_nstd_write(&[ch as u8], io::stdout())
}

/// Writes `str` to stdout.
/// Parameters:
///     `const char *const str` - String to write to stdout.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_io_write(str: *const c_char) -> c_int {
    static_nstd_write(CStr::from_ptr(str).to_bytes(), io::stdout())
}

/// Writes `str` to stdout with an additional newline.
/// Parameters:
///     `const char *const str` - String to write to stdout.
/// Returns: `int errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_io_write_line(str: *const c_char) -> c_int {
    let mut str = CStr::from_ptr(str).to_string_lossy().to_string();
    str.push('\n');
    static_nstd_write(str.as_bytes(), io::stdout())
}

/// Attempts to flush stderr.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_io_flush_err() -> c_int {
    match io::stderr().flush() {
        Ok(_) => 0,
        _ => 1,
    }
}

/// Writes a single character to stderr.
/// Parameters:
///     `const char ch` - Character to write.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_io_write_char_err(ch: c_char) -> c_int {
    static_nstd_write(&[ch as u8], io::stderr())
}

/// Writes `str` to stderr.
/// Parameters:
///     `const char *const str` - String to write to stderr.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_io_write_err(str: *const c_char) -> c_int {
    static_nstd_write(CStr::from_ptr(str).to_bytes(), io::stderr())
}

/// Writes `str` to stderr with an additional newline.
/// Parameters:
///     `const char *const str` - String to write to stderr.
/// Returns: `int errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_io_write_line_err(str: *const c_char) -> c_int {
    let mut str = CStr::from_ptr(str).to_string_lossy().to_string();
    str.push('\n');
    static_nstd_write(str.as_bytes(), io::stderr())
}

/// Reads a single character from stdin.
/// Parameters:
///     `int *errc` - Error code, returns as nonzero on error.
/// Returns: `char ch` - Character read from stdin.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_io_read_char(errc: *mut c_int) -> c_char {
    let mut byte = [0];
    match BufReader::new(io::stdin()).read_exact(&mut byte) {
        Ok(_) => {
            *errc = 0;
            byte[0] as c_char
        }
        _ => {
            *errc = 1;
            0
        }
    }
}

/// Reads from stdin and returns the read string.
/// Parameters:
///     `int *errc` - Error code, returns as nonzero on error.
/// Returns: `char *in` - String read from stdin.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_io_read(errc: *mut c_int) -> *mut c_char {
    let mut bytes = static_nstd_read(errc);
    match *errc {
        0 => {
            let mut len = bytes.len();
            while len > 0 && bytes[len - 1].is_ascii_whitespace() {
                bytes.pop();
                len -= 1;
            }
            bytes.push(0);
            CString::from_vec_unchecked(bytes).into_raw()
        }
        _ => ptr::null_mut(),
    }
}

/// Reads from stdin and returns the read string appending a newline to the end.
/// Parameters:
///     `int *errc` - Error code, returns as nonzero on error.
/// Returns: `char *in` - String read from stdin.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_io_read_line(errc: *mut c_int) -> *mut c_char {
    let mut bytes = static_nstd_read(errc);
    bytes.push(b'\0');
    CString::from_vec_unchecked(bytes).into_raw()
}

/// Frees memory allocated by `nstd_std_io_read` and `nstd_std_io_readline`.
/// Parameters:
///     `const char **str` - Pointer to the string returned from the read functions.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_io_free_read(str: *mut *mut c_char) {
    drop(CString::from_raw(*str));
    *str = ptr::null_mut();
}

/// Writes `bytes` to and flushes `buf`.
fn static_nstd_write<Stream: Write>(bytes: &[u8], stream: Stream) -> c_int {
    let mut buf = BufWriter::new(stream);
    match buf.write_all(bytes) {
        Ok(_) => match buf.flush() {
            Ok(_) => 0,
            _ => 1,
        },
        _ => 1,
    }
}

/// Reads `Vec<u8>` from stdin.
unsafe fn static_nstd_read(errc: *mut c_int) -> Vec<u8> {
    let mut string = String::new();
    *errc = match BufReader::new(io::stdin()).read_line(&mut string) {
        Ok(_) => 0,
        _ => 1,
    };
    string.into_bytes()
}
