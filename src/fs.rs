pub mod file;
use self::file::NSTDFile;
use crate::{
    collections::vec::*,
    io::{
        input_stream::NSTDInputStream, io_stream::NSTDIOStream, output_stream::NSTDOutputStream,
        stream::NSTDStream,
    },
};
use std::{
    ffi::{CStr, CString},
    fs::{self, OpenOptions},
    io::BufReader,
    os::raw::{c_char, c_int, c_void},
    path::Path,
};
pub const NSTD_FS_CREATE: usize = 0b00000001;
pub const NSTD_FS_READ: usize = 0b00000010;
pub const NSTD_FS_WRITE: usize = 0b00000100;
pub const NSTD_FS_APPEND: usize = 0b00001000;
pub const NSTD_FS_TRUNCATE: usize = 0b00010000;

/// Generates `nstd_fs_exists`, `nstd_fs_is_file` and `nstd_fs_is_dir` fns.
macro_rules! nstd_exists_fns {
    ($name: ident, $method: ident) => {
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(path: *const c_char) -> c_int {
            match CStr::from_ptr(path).to_str() {
                Ok(path) => Path::new(path).$method() as c_int,
                _ => 0,
            }
        }
    };
}
nstd_exists_fns!(nstd_fs_exists, exists);
nstd_exists_fns!(nstd_fs_is_file, is_file);
nstd_exists_fns!(nstd_fs_is_dir, is_dir);

/// Returns a vector of all a directory's contents.
/// NOTE: Memory allocated by this function should be freed with `nstd_fs_dir_contents_free`.
/// Parameters:
///     `const char *const dir` - The directory.
/// Returns: `NSTDVec contents` - The directory's contents.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_dir_contents(dir: *const c_char) -> NSTDVec {
    const ELEMENT_SIZE: usize = std::mem::size_of::<*const c_char>();
    match CStr::from_ptr(dir).to_str() {
        Ok(dir) => match fs::read_dir(dir) {
            Ok(iter_contents) => {
                let mut contents = nstd_collections_vec_new(ELEMENT_SIZE);
                if !contents.buffer.ptr.raw.is_null() {
                    for path_obj in iter_contents {
                        match path_obj {
                            Ok(entry) => match entry.file_name().into_string() {
                                Ok(name) => {
                                    let mut bytes = name.into_bytes();
                                    bytes.push(0);
                                    let raw = CString::from_vec_unchecked(bytes).into_raw();
                                    let raw_ptr = &raw as *const *mut c_char as *const c_void;
                                    nstd_collections_vec_push(&mut contents, raw_ptr);
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                    nstd_collections_vec_shrink(&mut contents);
                }
                contents
            }
            _ => NSTDVec::default(),
        },
        _ => NSTDVec::default(),
    }
}

/// Frees memory allocated by `nstd_fs_dir_contents`.
/// Parameters:
///     `NSTDVec *const contents` - A directory's contents.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_dir_contents_free(contents: &mut NSTDVec) -> c_int {
    for i in 0..contents.size {
        let element = nstd_collections_vec_get(contents, i) as *mut *mut c_char;
        drop(CString::from_raw(*element));
    }
    nstd_collections_vec_free(contents)
}

/// Creates a directory with the name `name`.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_create_dir(name: *const c_char) -> c_int {
    match CStr::from_ptr(name).to_str() {
        Ok(name) => fs::create_dir(name).is_err() as c_int,
        _ => 1,
    }
}

/// Creates a directory and all of it's parents if they are missing.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_create_dir_all(name: *const c_char) -> c_int {
    match CStr::from_ptr(name).to_str() {
        Ok(name) => fs::create_dir_all(name).is_err() as c_int,
        _ => 1,
    }
}

/// Removes an empty directory.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_remove_dir(name: *const c_char) -> c_int {
    match CStr::from_ptr(name).to_str() {
        Ok(name) => fs::remove_dir(name).is_err() as c_int,
        _ => 1,
    }
}

/// Removes a directory and all of it's contents.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_remove_dir_all(name: *const c_char) -> c_int {
    match CStr::from_ptr(name).to_str() {
        Ok(name) => fs::remove_dir_all(name).is_err() as c_int,
        _ => 1,
    }
}

/// Opens a file and returns the file stream. Files must be closed.
/// Parameters:
///     `const char *const name` - The name of the file.
///     `const NSTDUSize mask` - Bit mask defining how to open the file.
///         - Bit 1 - Create the file if it doesn't exist. Write bit must be set for this to work.
///         - Bit 2 - Read from the file.
///         - Bit 3 - Write to the file.
///         - Bit 4 - Append to the file.
///         - Bit 5 - Truncate the file.
/// Returns: `NSTDFile file` - The file stream.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_open(name: *const c_char, mask: usize) -> NSTDFile {
    let stream = NSTDStream { errc: 0 };
    let io_stream = NSTDIOStream {
        input_stream: NSTDInputStream {
            stream,
            read: Some(self::file::fs_istream_read),
            read_exact: Some(self::file::fs_istream_read_exact),
            read_until: Some(self::file::fs_istream_read_until),
            read_line: Some(self::file::fs_istream_read_line),
        },
        output_stream: NSTDOutputStream {
            stream,
            flush: Some(self::file::fs_ostream_flush),
            write: Some(self::file::fs_ostream_write),
        },
    };
    if let Ok(name) = CStr::from_ptr(name).to_str() {
        if let Ok(f) = OpenOptions::new()
            .create(mask & NSTD_FS_CREATE != 0)
            .read(mask & NSTD_FS_READ != 0)
            .write(mask & NSTD_FS_WRITE != 0)
            .append(mask & NSTD_FS_APPEND != 0)
            .truncate(mask & NSTD_FS_TRUNCATE != 0)
            .open(name)
        {
            return NSTDFile {
                io_stream,
                handle: Box::into_raw(Box::new(BufReader::new(f))),
            };
        }
    }
    NSTDFile {
        io_stream,
        handle: std::ptr::null_mut(),
    }
}

/// Frees a file stream and closes the file.
/// Parameters:
///     `NSTDFile *const file` - The file stream to free.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_close(file: &mut NSTDFile) {
    Box::from_raw(file.handle);
    file.handle = std::ptr::null_mut();
}
