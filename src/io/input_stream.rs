use crate::{
    collections::vec::NSTDVec, core::def::NSTDAny, io::stream::NSTDStream, string::NSTDString,
};

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
    /// Reads a line from this input stream into a string.
    /// Parameters:
    ///     `NSTDAny this` - A pointer to the owner of this stream.
    /// Returns: `NSTDString string` - Line read from the input stream.
    pub read_line: Option<unsafe extern "C" fn(NSTDAny) -> NSTDString>,
}
