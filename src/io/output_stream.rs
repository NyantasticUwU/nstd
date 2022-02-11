use crate::{
    core::{def::NSTDAny, slice::NSTDSlice},
    io::stream::NSTDStream,
};
use std::io::Write;

/// Represents a raw handle to an output stream.
pub type NSTDRawOutputStream = Box<Box<dyn Write>>;

/// Represents an output stream.
#[repr(C)]
pub struct NSTDOutputStream {
    /// The base stream.
    pub stream: NSTDStream,
    /// A raw handle to the output stream.
    pub ostream: NSTDRawOutputStream,
    /// Flushes this stream.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of the stream.
    pub flush: Option<unsafe extern "C" fn(NSTDAny)>,
    /// Writes a slice to this stream.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of the stream.
    ///     `const NSTDSlice *buff` - The buffer to write to this stream.
    pub write: Option<unsafe extern "C" fn(NSTDAny, &NSTDSlice)>,
}
