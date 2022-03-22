//! Output stream type.
use crate::{
    core::{def::NSTDAny, slice::NSTDSlice},
    io::stream::NSTDStream,
};

/// Represents an output stream.
#[repr(C)]
pub struct NSTDOutputStream {
    /// The base stream.
    pub stream: NSTDStream,
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
