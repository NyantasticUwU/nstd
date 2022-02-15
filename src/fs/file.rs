use crate::{
    collections::vec::NSTDVec,
    core::{
        def::{NSTDAny, NSTDChar},
        slice::NSTDSlice,
    },
    io::{
        input_stream::NSTDInputStream, io_stream::NSTDIOStream, output_stream::NSTDOutputStream,
        stream::NSTDStream,
    },
    string::NSTDString,
};
use std::{
    ffi::CStr,
    fs::File,
    io::{prelude::*, BufReader},
};
pub const NSTD_FS_CREATE: usize = 0b00000001;
pub const NSTD_FS_READ: usize = 0b00000010;
pub const NSTD_FS_WRITE: usize = 0b00000100;
pub const NSTD_FS_APPEND: usize = 0b00001000;
pub const NSTD_FS_TRUNCATE: usize = 0b00010000;

/// Represents a raw handle to a file.
pub type NSTDFileHandle = *mut BufReader<File>;

/// Represents a file stream.
#[repr(C)]
pub struct NSTDFile {
    /// The input/output stream.
    pub io_stream: NSTDIOStream,
    /// The handle to the file.
    pub handle: NSTDFileHandle,
}

/// Opens a file and returns the file stream. Files must be closed.
/// Parameters:
///     `const NSTDChar *const name` - The name of the file.
///     `const NSTDUSize mask` - Bit mask defining how to open the file.
///         - Bit 1 - Create the file if it doesn't exist. Write bit must be set for this to work.
///         - Bit 2 - Read from the file.
///         - Bit 3 - Write to the file.
///         - Bit 4 - Append to the file.
///         - Bit 5 - Truncate the file.
/// Returns: `NSTDFile file` - The file stream.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_file_open(name: *const NSTDChar, mask: usize) -> NSTDFile {
    let stream = NSTDStream { errc: 0 };
    let io_stream = NSTDIOStream {
        input_stream: NSTDInputStream {
            stream,
            read: Some(fs_istream_read),
            read_exact: Some(fs_istream_read_exact),
            read_until: Some(fs_istream_read_until),
            read_line: Some(fs_istream_read_line),
        },
        output_stream: NSTDOutputStream {
            stream,
            flush: Some(fs_ostream_flush),
            write: Some(fs_ostream_write),
        },
    };
    if let Ok(name) = CStr::from_ptr(name).to_str() {
        if let Ok(f) = File::options()
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
pub unsafe extern "C" fn nstd_fs_file_close(file: &mut NSTDFile) {
    Box::from_raw(file.handle);
    file.handle = std::ptr::null_mut();
}

/// Reads from this file stream.
pub(crate) unsafe extern "C" fn fs_istream_read(this: NSTDAny) -> NSTDVec {
    let this = this as *mut NSTDFile;
    let mut buff = Vec::new();
    if (*(*this).handle).read_to_end(&mut buff).is_err() {
        (*this).io_stream.input_stream.stream.errc = 1;
    }
    NSTDVec::from(buff)
}

/// Reads a specific number of bytes from this file stream.
pub(crate) unsafe extern "C" fn fs_istream_read_exact(this: NSTDAny, count: usize) -> NSTDVec {
    let this = this as *mut NSTDFile;
    let mut buff = Vec::new();
    buff.resize(count, 0);
    if (*(*this).handle).read_exact(&mut buff).is_err() {
        (*this).io_stream.input_stream.stream.errc = 1;
    }
    NSTDVec::from(buff)
}

/// Reads bytes from this file stream until `delimiter` is reached.
pub(crate) unsafe extern "C" fn fs_istream_read_until(this: NSTDAny, delimiter: u8) -> NSTDVec {
    let this = this as *mut NSTDFile;
    let mut buff = Vec::new();
    if (*(*this).handle).read_until(delimiter, &mut buff).is_err() {
        (*this).io_stream.input_stream.stream.errc = 1;
    }
    NSTDVec::from(buff)
}

/// Reads a line from this file stream.
pub(crate) unsafe extern "C" fn fs_istream_read_line(this: NSTDAny) -> NSTDString {
    let this = this as *mut NSTDFile;
    let mut buff = String::new();
    if (*(*this).handle).read_line(&mut buff).is_err() {
        (*this).io_stream.input_stream.stream.errc = 1;
    }
    NSTDString::from(buff.into_bytes())
}

/// Flushes a file stream.
#[inline]
pub(crate) unsafe extern "C" fn fs_ostream_flush(this: NSTDAny) {
    let this = this as *mut NSTDFile;
    if (*(*this).handle).get_mut().flush().is_err() {
        (*this).io_stream.output_stream.stream.errc = 1;
    }
}

/// Writes to a file stream.
#[inline]
pub(crate) unsafe extern "C" fn fs_ostream_write(this: NSTDAny, buff: &NSTDSlice) {
    let this = this as *mut NSTDFile;
    if (*(*this).handle)
        .get_mut()
        .write_all(buff.as_byte_slice())
        .is_err()
    {
        (*this).io_stream.output_stream.stream.errc = 1;
    }
}
