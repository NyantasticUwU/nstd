//! Directory management & file I/O.
pub mod file;
use crate::{
    core::def::{NSTDBool, NSTDChar, NSTDErrorCode},
    string::NSTDString,
    vec::*,
};
use std::{ffi::CStr, path::Path, ptr::addr_of_mut};

/// Generates `nstd_fs_exists`, `nstd_fs_is_file` and `nstd_fs_is_dir` fns.
macro_rules! nstd_exists_fns {
    ($name: ident, $method: ident) => {
        ///
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(path: *const NSTDChar) -> NSTDBool {
            if let Ok(path) = CStr::from_ptr(path).to_str() {
                return NSTDBool::from(Path::new(path).$method());
            }
            NSTDBool::NSTD_BOOL_FALSE
        }
    };
}
nstd_exists_fns!(nstd_fs_exists, exists);
nstd_exists_fns!(nstd_fs_is_file, is_file);
nstd_exists_fns!(nstd_fs_is_dir, is_dir);

/// Returns a vector of all a directory's contents.
///
/// # Note
///
/// Memory allocated by this function should be freed with `nstd_fs_dir_contents_free`.
///
/// # Parameters
///
/// - `const NSTDChar *const dir` - The directory.
///
/// # Returns
///
/// `NSTDVec contents` - An `NSTDVec` of `NSTDString`.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_dir_contents(dir: *const NSTDChar) -> NSTDVec {
    const ELEMENT_SIZE: usize = std::mem::size_of::<*const NSTDChar>();
    if let Ok(dir) = CStr::from_ptr(dir).to_str() {
        if let Ok(iter_contents) = std::fs::read_dir(dir) {
            let mut contents = nstd_vec_new(ELEMENT_SIZE);
            if !contents.buffer.ptr.raw.is_null() {
                for path_obj in iter_contents {
                    if let Ok(entry) = path_obj {
                        if let Ok(name) = entry.file_name().into_string() {
                            let mut string = NSTDString::from(name.as_bytes());
                            let strptr = addr_of_mut!(string).cast();
                            nstd_vec_push(&mut contents, strptr);
                        }
                    }
                }
                nstd_vec_shrink(&mut contents);
            }
            return contents;
        }
    }
    NSTDVec::default()
}

/// Frees memory allocated by `nstd_fs_dir_contents`.
///
/// # Parameters
///
/// - `NSTDVec *const contents` - A directory's contents.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_dir_contents_free(contents: &mut NSTDVec) -> NSTDErrorCode {
    for i in 0..contents.size {
        let element = nstd_vec_get(contents, i) as *mut NSTDString;
        crate::string::nstd_string_free(&mut *element);
    }
    nstd_vec_free(contents)
}

/// Creates a directory with the name `name`.
///
/// # Parameters
///
/// - `const NSTDChar *const name` - The name of the directory.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_create_dir(name: *const NSTDChar) -> NSTDErrorCode {
    if let Ok(name) = CStr::from_ptr(name).to_str() {
        return std::fs::create_dir(name).is_err() as NSTDErrorCode;
    }
    1
}

/// Creates a directory and all of it's parents if they are missing.
///
/// # Parameters
///
/// - `const NSTDChar *const name` - The name of the directory.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_create_dir_all(name: *const NSTDChar) -> NSTDErrorCode {
    if let Ok(name) = CStr::from_ptr(name).to_str() {
        return std::fs::create_dir_all(name).is_err() as NSTDErrorCode;
    }
    1
}

/// Removes an empty directory.
///
/// # Parameters
///
/// - `const NSTDChar *const name` - The name of the directory.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_remove_dir(name: *const NSTDChar) -> NSTDErrorCode {
    if let Ok(name) = CStr::from_ptr(name).to_str() {
        return std::fs::remove_dir(name).is_err() as NSTDErrorCode;
    }
    1
}

/// Removes a directory and all of it's contents.
///
/// # Parameters
///
/// - `const NSTDChar *const name` - The name of the directory.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_remove_dir_all(name: *const NSTDChar) -> NSTDErrorCode {
    if let Ok(name) = CStr::from_ptr(name).to_str() {
        return std::fs::remove_dir_all(name).is_err() as NSTDErrorCode;
    }
    1
}
