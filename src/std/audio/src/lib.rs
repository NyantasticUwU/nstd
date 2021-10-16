use cpal::{
    traits::*, BufferSize, BuildStreamError, Device, Host, Sample, SampleFormat, SampleRate,
    Stream, StreamConfig,
};
use nstd_fs::NSTDFile;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{
    ffi::CString,
    fs::File,
    io::BufReader,
    os::raw::{c_char, c_float, c_int, c_void},
    ptr,
};

/// Represents an audio host.
pub type NSTDAudioHost = *mut c_void;

/// Represents an audio device.
pub type NSTDAudioDevice = *mut c_void;

/// Represents an audio stream.
pub type NSTDAudioStream = *mut c_void;

/// Represents an audio sample format.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDAudioSampleFormat {
    INT16,
    UINT16,
    FLOAT32,
}

/// Represents a stream config.
/// NOTE: `buffer_size` will be set to 0 for default.
#[repr(C)]
pub struct NSTDAudioStreamConfig {
    pub channels: u16,
    pub sample_rate: u32,
    pub buffer_size: u32,
    pub format: NSTDAudioSampleFormat,
}

/// Represents an audio play stream.
#[repr(C)]
pub struct NSTDAudioPlayStream {
    pub stream: *mut OutputStream,
    pub handle: *mut OutputStreamHandle,
}
impl Default for NSTDAudioPlayStream {
    fn default() -> Self {
        Self {
            stream: ptr::null_mut(),
            handle: ptr::null_mut(),
        }
    }
}

/// Represents an audio sink.
pub type NSTDAudioSink = *mut Sink;

/// Gets the default audio host.
/// Returns: `NSTDAudioHost host` - The default audio host.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_host_default() -> NSTDAudioHost {
    Box::into_raw(Box::new(cpal::default_host())) as NSTDAudioHost
}

/// Generates `nstd_std_audio_host_default_*_device` functions.
macro_rules! generate_default_device {
    ($name: ident, $method: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(host: NSTDAudioHost) -> NSTDAudioDevice {
            let host = &*(host as *mut Host);
            match host.$method() {
                Some(device) => Box::into_raw(Box::new(device)) as NSTDAudioDevice,
                _ => ptr::null_mut(),
            }
        }
    };
}
generate_default_device!(
    nstd_std_audio_host_default_input_device,
    default_input_device
);
generate_default_device!(
    nstd_std_audio_host_default_output_device,
    default_output_device
);

/// Frees a host's memory.
/// Parameters:
///     `NSTDAudioHost *host` - Pointer to an audio host.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_host_free(host: *mut NSTDAudioHost) {
    Box::from_raw(*host as *mut Host);
    *host = ptr::null_mut();
}

/// Gets the name of a device.
/// Parameters:
///     `NSTDAudioDevice device` - The device.
/// Returns: `char *name` - The device name.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_device_name(device: NSTDAudioDevice) -> *mut c_char {
    let device = &*(device as *mut Device);
    match device.name() {
        Ok(name) => {
            let mut bytes = name.into_bytes();
            bytes.push(0);
            CString::from_vec_unchecked(bytes).into_raw()
        }
        _ => ptr::null_mut(),
    }
}

/// Frees a device name.
/// Parameters:
///     `const char **name` - A device name.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_device_free_name(name: *mut *mut c_char) {
    CString::from_raw(*name);
    *name = ptr::null_mut();
}

/// Generates `nstd_std_audio_device_default_*_config` functions.
macro_rules! generate_device_default_config {
    ($name: ident, $method: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            device: NSTDAudioDevice,
            config: *mut NSTDAudioStreamConfig,
        ) -> c_int {
            let device = &*(device as *mut Device);
            match device.$method() {
                Ok(cpal_supported_config) => {
                    let format = cpal_supported_config.sample_format();
                    let cpal_config = cpal_supported_config.config();
                    *config = NSTDAudioStreamConfig {
                        channels: cpal_config.channels,
                        sample_rate: cpal_config.sample_rate.0,
                        buffer_size: match cpal_config.buffer_size {
                            BufferSize::Default => 0,
                            BufferSize::Fixed(size) => size,
                        },
                        format: match format {
                            SampleFormat::I16 => NSTDAudioSampleFormat::INT16,
                            SampleFormat::U16 => NSTDAudioSampleFormat::UINT16,
                            SampleFormat::F32 => NSTDAudioSampleFormat::FLOAT32,
                        },
                    };
                    0
                }
                _ => 1,
            }
        }
    };
}
generate_device_default_config!(
    nstd_std_audio_device_default_input_config,
    default_input_config
);
generate_device_default_config!(
    nstd_std_audio_device_default_output_config,
    default_output_config
);

/// Generates `nstd_std_audio_device_build_*_stream` functions.
macro_rules! generate_device_build_stream {
    ($name: ident, $func: ident, $ptr_ty: ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            device: NSTDAudioDevice,
            config: *const NSTDAudioStreamConfig,
            format: NSTDAudioSampleFormat,
            callback: extern "C" fn($ptr_ty, usize),
            err_callback: extern "C" fn(),
        ) -> NSTDAudioStream {
            let device = &*(device as *mut Device);
            let config = StreamConfig {
                channels: (*config).channels,
                sample_rate: SampleRate((*config).sample_rate),
                buffer_size: match (*config).buffer_size {
                    0 => BufferSize::Default,
                    size => BufferSize::Fixed(size),
                },
            };
            match match format {
                NSTDAudioSampleFormat::INT16 => {
                    $func::<i16>(device, &config, callback, err_callback)
                }
                NSTDAudioSampleFormat::UINT16 => {
                    $func::<u16>(device, &config, callback, err_callback)
                }
                NSTDAudioSampleFormat::FLOAT32 => {
                    $func::<f32>(device, &config, callback, err_callback)
                }
            } {
                Ok(stream) => match stream.play() {
                    Ok(_) => Box::into_raw(Box::new(stream)) as NSTDAudioStream,
                    _ => ptr::null_mut(),
                },
                _ => ptr::null_mut(),
            }
        }
    };
}
generate_device_build_stream!(
    nstd_std_audio_device_build_input_stream,
    build_input_stream,
    *const c_void
);
generate_device_build_stream!(
    nstd_std_audio_device_build_output_stream,
    build_output_stream,
    *mut c_void
);

/// Frees a device.
/// Parameters:
///     `NSTDAudioDevice *device` - Pointer to a device.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_device_free(device: *mut NSTDAudioDevice) {
    Box::from_raw(*device as *mut Device);
    *device = ptr::null_mut();
}

/// Generates `nstd_std_audio_stream_*` functions.
macro_rules! generate_stream_play_pause {
    ($name: ident, $method: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(stream: NSTDAudioStream) -> c_int {
            let stream = &*(stream as *mut Stream);
            match stream.$method() {
                Ok(_) => 0,
                _ => 1,
            }
        }
    };
}
generate_stream_play_pause!(nstd_std_audio_stream_play, play);
generate_stream_play_pause!(nstd_std_audio_stream_pause, pause);

/// Frees an audio stream
/// Parameters:
///     `NSTDAudioStream *stream` - Pointer to an audio stream.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_stream_free(stream: *mut NSTDAudioStream) {
    Box::from_raw(*stream as *mut Stream);
    *stream = ptr::null_mut();
}

/// Generates `build_*_stream` functions.
macro_rules! generate_build_stream {
    ($name: ident, $ptr_method: ident, $cbptr_ty: ty, $slice_ty: ty) => {
        #[inline]
        fn $name<T: Sample>(
            device: &Device,
            config: &StreamConfig,
            callback: extern "C" fn($cbptr_ty, usize),
            err_callback: extern "C" fn(),
        ) -> Result<Stream, BuildStreamError> {
            device.$name(
                &config,
                move |data: $slice_ty, _| callback(data.$ptr_method() as $cbptr_ty, data.len()),
                move |_| err_callback(),
            )
        }
    };
}
generate_build_stream!(build_input_stream, as_ptr, *const c_void, &[T]);
generate_build_stream!(build_output_stream, as_mut_ptr, *mut c_void, &mut [T]);

/// Creates a play stream.
/// Returns: `NSTDAudioPlayStream stream` - The new play stream.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_play_stream_new() -> NSTDAudioPlayStream {
    match OutputStream::try_default() {
        Ok((stream, handle)) => NSTDAudioPlayStream {
            stream: Box::into_raw(Box::new(stream)),
            handle: Box::into_raw(Box::new(handle)),
        },
        _ => NSTDAudioPlayStream::default(),
    }
}

/// Frees a play stream.
/// Parameters:
///     `NSTDAudioPlayStream *stream` - The play stream.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_play_stream_free(stream: &mut NSTDAudioPlayStream) {
    Box::from_raw(stream.stream);
    Box::from_raw(stream.handle);
    stream.stream = ptr::null_mut();
    stream.handle = ptr::null_mut();
}

/// Creates a new audio sink.
/// Parameters:
///     `const NSTDAudioPlayStream *const stream` - The stream to create the sink on.
/// Returns: `NSTDAudioSink sink` - The new audio sink.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_sink_new(stream: &NSTDAudioPlayStream) -> NSTDAudioSink {
    match Sink::try_new(&*stream.handle) {
        Ok(sink) => Box::into_raw(Box::new(sink)),
        _ => ptr::null_mut(),
    }
}

/// Appends audio to a sink from a file.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
///     `NSTDFile file` - The audio file.
///     `const int should_loop` - Nonzero if the audio should be looped.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_sink_append_from_file(
    sink: NSTDAudioSink,
    file: NSTDFile,
    should_loop: c_int,
) -> c_int {
    let file = &*(file as *mut File);
    let buf = BufReader::new(file);
    match should_loop {
        0 => match Decoder::new(buf) {
            Ok(decoder) => {
                (*sink).append(decoder);
                0
            }
            _ => 1,
        },
        _ => match Decoder::new_looped(buf) {
            Ok(decoder) => {
                (*sink).append(decoder);
                0
            }
            _ => 1,
        },
    }
}

/// Plays an audio sink.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_sink_play(sink: NSTDAudioSink) {
    (*sink).play();
}

/// Pauses an audio sink.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_sink_pause(sink: NSTDAudioSink) {
    (*sink).pause();
}

/// Checks if an audio sink is paused.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
/// Returns: `int is_paused` - Whether or not the audio sink is paused.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_sink_is_paused(sink: NSTDAudioSink) -> c_int {
    (*sink).is_paused() as c_int
}

/// Stops audio playback for a sink by clearing it's queue.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_sink_stop(sink: NSTDAudioSink) {
    (*sink).stop();
}

/// Sleeps the current thread until all sounds in the sink are done playing.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_sink_sleep_until_end(sink: NSTDAudioSink) {
    (*sink).sleep_until_end();
}

/// Returns the volume of the audio sink.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
/// Returns: `float volume` - The volume of the sink.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_sink_get_volume(sink: NSTDAudioSink) -> c_float {
    (*sink).volume()
}

/// Sets the volume of the audio sink.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
///     `const float volume` - The volume of the sink.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_sink_set_volume(sink: NSTDAudioSink, volume: c_float) {
    (*sink).set_volume(volume);
}

/// Gets the number of audio sources currently in a sink.
/// Parameters:
///     `NSTDAudioSink sink` - The audio sink.
/// Returns: `NSTDSize size` - The number of audio sources in an audio sink.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_sink_length(sink: NSTDAudioSink) -> usize {
    (*sink).len()
}

/// Detaches a sink from it's thread while freeing its memory.
/// Parameters:
///     `NSTDAudioSink *sink` - The audio sink.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_sink_detach(sink: &mut NSTDAudioSink) {
    let boxed_sink = Box::from_raw(*sink);
    boxed_sink.detach();
    *sink = ptr::null_mut();
}

/// Frees an audio sink.
/// Parameters:
///     `NSTDAudioSink *sink` - The audio sink.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_audio_sink_free(sink: &mut NSTDAudioSink) {
    Box::from_raw(*sink);
    *sink = ptr::null_mut();
}
