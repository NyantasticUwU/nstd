use crate::{
    collections::vec::NSTDVec,
    core::{def::NSTDAny, slice::NSTDSlice},
    io::io_stream::NSTDIOStream,
    string::NSTDString,
};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

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
