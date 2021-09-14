use std::{
    ffi::{CStr, CString},
    io::{self, BufRead, BufReader, BufWriter, Write},
    os::raw::{c_char, c_int},
    ptr,
};

/// Writes a single character to stdout.
/// Parameters:
///     `const char ch` - Character to write.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_io_writechar(ch: c_char) -> c_int {
    static_nstd_write(&[ch as u8])
}

/// Writes `str` to stdout.
/// Parameters:
///     `const char *const str` - String to write to stdout.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_io_write(str: *const c_char) -> c_int {
    static_nstd_write(CStr::from_ptr(str).to_bytes())
}

/// Writes `str` to stdout with an additional newline.
/// Parameters:
///     `const char *const str` - String to write to stdout.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_io_writeline(str: *const c_char) -> c_int {
    let mut str = CStr::from_ptr(str).to_string_lossy().to_string();
    str.push('\n');
    static_nstd_write(str.as_bytes())
}

/// Reads a single character from stdin.
/// Parameters:
///     `int *const errc` - Error code, returns as nonzero on error.
/// Returns: `char ch` - Character read from stdin.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_io_readchar(errc: *mut c_int) -> c_char {
    static_nstd_read(errc)[0] as c_char
}

/// Reads from stdin and returns the read string.
/// Parameters:
///     `int *const errc` - Error code, returns as nonzero on error.
/// Returns: `char *in` - String read from stdin.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_io_read(errc: *mut c_int) -> *mut c_char {
    let mut bytes = static_nstd_read(errc);
    match (bytes.last_mut(), *errc) {
        (Some(byte), 0) => {
            *byte = b'\0';
            CString::from_vec_unchecked(bytes).into_raw()
        }
        _ => {
            *errc = 1;
            ptr::null_mut()
        }
    }
}

/// Reads from stdin and returns the read string appending a newline to the end.
/// Parameters:
///     `int *const errc` - Error code, returns as nonzero on error.
/// Returns: `char *in` - String read from stdin.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_io_readline(errc: *mut c_int) -> *mut c_char {
    let mut bytes = static_nstd_read(errc);
    bytes.push(b'\0');
    CString::from_vec_unchecked(bytes).into_raw()
}

/// Frees memory allocated by `nstd_std_io_read` and `nstd_std_io_readline`.
/// Parameters:
///     `const char **str` - Pointer to the string returned from the read functions.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_io_free_read(str: *mut *mut c_char) {
    CString::from_raw(*str);
    *str = ptr::null_mut();
}

/// Writes `bytes` to and flushes `buf`.
fn static_nstd_write(bytes: &[u8]) -> c_int {
    let mut buf = BufWriter::new(io::stdout());
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
