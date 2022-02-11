use crate::core::def::NSTDErrorCode;

/// An interface that represents a data stream.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct NSTDStream {
    /// Set to nonzero if an error has occurred on the stream.
    pub errc: NSTDErrorCode,
}
