//! Common audio-related types.

/// Represents an audio sample format.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDAudioSampleFormat {
    /// Signed 16-bit integer.
    INT16,
    /// Unsigned 16-bit integer.
    UINT16,
    /// 32-bit float.
    FLOAT32,
}

/// Represents a stream config.
///
/// # Note
///
/// `buffer_size` will be set to 0 for default.
#[repr(C)]
pub struct NSTDAudioStreamConfig {
    /// The number of channels this audio stream owns.
    pub channels: u16,
    /// The sample rate of the audio stream.
    pub sample_rate: u32,
    /// The size of the sample buffer.
    pub buffer_size: u32,
    /// The data format of each sample.
    pub format: NSTDAudioSampleFormat,
}
