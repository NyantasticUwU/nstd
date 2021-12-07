use nstd_collections::vec::*;
use std::{
    ffi::{CStr, CString},
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    os::raw::{c_char, c_int, c_longlong, c_void},
    path::Path,
    ptr, slice,
};
#[cfg(feature = "deps")]
pub mod deps {
    pub use nstd_collections;
}
pub const NSTD_STD_FS_CREATE: usize = 0b00000001;
pub const NSTD_STD_FS_READ: usize = 0b00000010;
pub const NSTD_STD_FS_WRITE: usize = 0b00000100;
pub const NSTD_STD_FS_APPEND: usize = 0b00001000;
pub const NSTD_STD_FS_TRUNCATE: usize = 0b00010000;

/// Represents a file handle.
pub type NSTDFile = *mut File;

/// Generates `nstd_std_fs_exists`, `nstd_std_fs_is_file` and `nstd_std_fs_is_dir` fns.
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
nstd_exists_fns!(nstd_std_fs_exists, exists);
nstd_exists_fns!(nstd_std_fs_is_file, is_file);
nstd_exists_fns!(nstd_std_fs_is_dir, is_dir);

/// Returns a vector of all a directory's contents.
/// NOTE: Memory allocated by this function should be freed with `nstd_std_fs_dir_contents_free`.
/// Parameters:
///     `const char *const dir` - The directory.
/// Returns: `NSTDVec contents` - The directory's contents.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_dir_contents(dir: *const c_char) -> NSTDVec {
    const ELEMENT_SIZE: usize = std::mem::size_of::<*const c_char>();
    match CStr::from_ptr(dir).to_str() {
        Ok(dir) => match fs::read_dir(dir) {
            Ok(iter_contents) => {
                let mut contents = nstd_std_collections_vec_new(ELEMENT_SIZE);
                if !contents.data.is_null() {
                    for path_obj in iter_contents {
                        match path_obj {
                            Ok(entry) => match entry.file_name().into_string() {
                                Ok(name) => {
                                    let mut bytes = name.into_bytes();
                                    bytes.push(0);
                                    let raw = CString::from_vec_unchecked(bytes).into_raw();
                                    let raw_ptr = &raw as *const *mut c_char as *const c_void;
                                    nstd_std_collections_vec_push(&mut contents, raw_ptr);
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                    nstd_std_collections_vec_shrink(&mut contents);
                }
                contents
            }
            _ => NSTDVec::default(),
        },
        _ => NSTDVec::default(),
    }
}

/// Frees memory allocated by `nstd_std_fs_dir_contents`.
/// Parameters:
///     `NSTDVec *const contents` - A directory's contents.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_dir_contents_free(contents: &mut NSTDVec) -> c_int {
    for i in 0..contents.size {
        let element = nstd_std_collections_vec_get(contents, i) as *mut *mut c_char;
        drop(CString::from_raw(*element));
    }
    nstd_std_collections_vec_free(contents)
}

/// Creates a directory with the name `name`.
/// Parameters:
///     `const char *const name` - The name of the directory.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_create_dir(name: *const c_char) -> c_int {
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
pub unsafe extern "C" fn nstd_std_fs_create_dir_all(name: *const c_char) -> c_int {
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
pub unsafe extern "C" fn nstd_std_fs_remove_dir(name: *const c_char) -> c_int {
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
pub unsafe extern "C" fn nstd_std_fs_remove_dir_all(name: *const c_char) -> c_int {
    match CStr::from_ptr(name).to_str() {
        Ok(name) => fs::remove_dir_all(name).is_err() as c_int,
        _ => 1,
    }
}

/// Opens a file and returns the file handle. Files must be closed.
/// Parameters:
///     `const char *const name` - The name of the file.
///     `const NSTDUSize mask` - Bit mask defining how to open the file.
///         - Bit 1 - Create the file if it doesn't exist. Write bit must be set for this to work.
///         - Bit 2 - Read from the file.
///         - Bit 3 - Write to the file.
///         - Bit 4 - Append to the file.
///         - Bit 5 - Truncate the file.
/// Returns: `NSTDFile file` - A handle to the opened file.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_open(name: *const c_char, mask: usize) -> NSTDFile {
    if let Ok(name) = CStr::from_ptr(name).to_str() {
        if let Ok(f) = OpenOptions::new()
            .create(mask & NSTD_STD_FS_CREATE != 0)
            .read(mask & NSTD_STD_FS_READ != 0)
            .write(mask & NSTD_STD_FS_WRITE != 0)
            .append(mask & NSTD_STD_FS_APPEND != 0)
            .truncate(mask & NSTD_STD_FS_TRUNCATE != 0)
            .open(name)
        {
            return Box::into_raw(Box::new(f));
        }
    }
    ptr::null_mut()
}

/// Writes a string buffer to the specified file.
/// Parameters:
///     `NSTDFile file` - The file to write to.
///     `const char *const buf` - The buffer to write.
/// Returns: `int errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_write(file: NSTDFile, buf: *const c_char) -> c_int {
    if let Ok(buf) = CStr::from_ptr(buf).to_str() {
        if let Ok(_) = (*file).write_all(buf.as_bytes()) {
            return 0;
        }
    }
    1
}

/// Reads file into string.
/// Parameters:
///     `NSTDFile file` - The file to read from.
/// Returns: `char *contents` - The file contents, null on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_read(file: NSTDFile) -> *mut c_char {
    let mut buf = String::new();
    if let Ok(_) = (*file).read_to_string(&mut buf) {
        buf.push('\0');
        CString::from_vec_unchecked(buf.into_bytes()).into_raw()
    } else {
        ptr::null_mut()
    }
}

/// Frees data from `nstd_std_fs_read`.
/// Parameters:
///     `char **contents` - Pointer to the string.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_free_read(contents: *mut *mut c_char) {
    drop(CString::from_raw(*contents));
    *contents = ptr::null_mut();
}

/// Reads raw data from a file.
/// Parameters:
///     `NSTDFile file` - The file to read from.
///     `NSTDUSize *const size` - Returns as number of bytes read.
/// Returns: `NSTDByte *data` - The raw file data.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_read_raw(file: NSTDFile, size: *mut usize) -> *mut u8 {
    let mut buf = Vec::new();
    match (*file).read_to_end(&mut buf) {
        Ok(len) => {
            *size = len;
            Box::into_raw(buf.into_boxed_slice()) as *mut u8
        }
        _ => {
            *size = 0;
            ptr::null_mut()
        }
    }
}

/// Frees raw data that has been read from a file.
/// Parameters:
///     `NSTDByte **const data` - The data to be freed.
///     `const NSTDUSize size` - Number of bytes to free.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_free_raw(data: *mut *mut u8, size: usize) {
    Box::from_raw(slice::from_raw_parts_mut(*data, size) as *mut [u8]);
    *data = ptr::null_mut();
}

/// Sets the position of the stream pointer from the current pos of the stream pointer.
/// Parameters:
///     `NSTDFile file` - The file handle.
///     `long long pos` - The position to set the stream pointer to.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_seek(file: NSTDFile, pos: c_longlong) -> c_int {
    static_file_seek(file, SeekFrom::Current(pos))
}

/// Sets the position of the stream pointer from the start of a file.
/// Parameters:
///     `NSTDFile file` - The file handle.
///     `long long pos` - The position to set the stream pointer to.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_seek_from_start(file: NSTDFile, pos: c_longlong) -> c_int {
    static_file_seek(file, SeekFrom::Start(pos as u64))
}

/// Sets the position of the stream pointer from the end of a file.
/// Parameters:
///     `NSTDFile file` - The file handle.
///     `long long pos` - The position to set the stream pointer to.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_seek_from_end(file: NSTDFile, pos: c_longlong) -> c_int {
    static_file_seek(file, SeekFrom::End(pos))
}

/// Rewinds the stream pointer to the start of the file.
/// Parameters:
///     `NSTDFile file` - The file handle.
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_rewind(file: NSTDFile) -> c_int {
    static_file_seek(file, SeekFrom::Start(0))
}

/// Closes a file.
/// Parameters:
///     `NSTDFile *handle` - The handle to the file.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_std_fs_close(handle: &mut NSTDFile) {
    Box::from_raw(*handle);
    *handle = ptr::null_mut();
}

/// Sets the position of a file stream pointer.
/// Parameters:
///     `file: &mut File` - The file.
///     `pos: SeekFrom` - The position.
/// Returns: `errc: c_int` - Nonzero on error.
#[inline]
unsafe fn static_file_seek(file: NSTDFile, pos: SeekFrom) -> c_int {
    match (*file).seek(pos) {
        Ok(_) => 0,
        _ => 1,
    }
}
