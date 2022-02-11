use crate::{
    collections::vec::NSTDVec, core::def::NSTDAny, io::stream::NSTDStream, string::NSTDString,
};
use std::io::BufRead;

/// Represents a raw handle to an input stream.
pub type NSTDRawInputStream = Box<Box<dyn BufRead>>;

/// Represents an input stream.
#[repr(C)]
pub struct NSTDInputStream {
    /// The base stream.
    pub stream: NSTDStream,
    /// A raw handle to the input stream.
    pub istream: NSTDRawInputStream,
    /// Reads data from this input stream into a vector.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of this stream.
    /// Returns: `NSTDVec buff` - Bytes read from the input stream.
    pub read: Option<unsafe extern "C" fn(NSTDAny) -> NSTDVec>,
    /// Reads a line from this input stream into a string.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of this stream.
    /// Returns: `NSTDString string` - Line read from the input stream.
    pub read_line: Option<unsafe extern "C" fn(NSTDAny) -> NSTDString>,
}
