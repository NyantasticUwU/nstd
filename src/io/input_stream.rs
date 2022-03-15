use crate::{core::def::NSTDAny, io::stream::NSTDStream, string::NSTDString, vec::NSTDVec};

/// Represents an input stream.
#[repr(C)]
pub struct NSTDInputStream {
    /// The base stream.
    pub stream: NSTDStream,
    /// Reads data from this input stream into a vector.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of this stream.
    /// Returns: `NSTDVec buff` - Bytes read from the input stream.
    pub read: Option<unsafe extern "C" fn(NSTDAny) -> NSTDVec>,
    /// Reads a specific number of bytes from this input stream into a vector.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of this stream.
    ///     `NSTDUSize count` - The number of bytes to read.
    /// Returns: `NSTDVec buff` - Bytes read from the input stream.
    pub read_exact: Option<unsafe extern "C" fn(NSTDAny, usize) -> NSTDVec>,
    /// Reads from this input stream until `delimiter` is reached.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of this stream.
    ///     `NSTDByte delimiter` - The delimiter.
    /// Returns: `NSTDVec buff` - Bytes read from the input stream.
    pub read_until: Option<unsafe extern "C" fn(NSTDAny, u8) -> NSTDVec>,
    /// Reads a line from this input stream into a string.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of this stream.
    /// Returns: `NSTDString string` - Line read from the input stream.
    pub read_line: Option<unsafe extern "C" fn(NSTDAny) -> NSTDString>,
}
