pub mod file;
use crate::collections::vec::*;
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int, c_void},
    path::Path,
};

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
        Ok(dir) => match std::fs::read_dir(dir) {
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
        Ok(name) => std::fs::create_dir(name).is_err() as c_int,
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
        Ok(name) => std::fs::create_dir_all(name).is_err() as c_int,
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
        Ok(name) => std::fs::remove_dir(name).is_err() as c_int,
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
        Ok(name) => std::fs::remove_dir_all(name).is_err() as c_int,
        _ => 1,
    }
}
