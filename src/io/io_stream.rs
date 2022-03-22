//! Stream type for both input and output.
use crate::io::{input_stream::NSTDInputStream, output_stream::NSTDOutputStream};

/// Represents both an input and an output stream.
#[repr(C)]
pub struct NSTDIOStream {
    /// The input stream.
    pub input_stream: NSTDInputStream,
    /// The output stream.
    pub output_stream: NSTDOutputStream,
}
