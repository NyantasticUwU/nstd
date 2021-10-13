use cpal::{
    traits::*, BufferSize, BuildStreamError, Device, Host, Sample, SampleFormat, SampleRate,
    Stream, StreamConfig,
};
use std::{
    ffi::CString,
    os::raw::{c_char, c_int, c_void},
    ptr,
};

/// Represents an audio host.
type NSTDAudioHost = *mut c_void;

/// Represents an audio device.
type NSTDAudioDevice = *mut c_void;

/// Represents an audio stream.
type NSTDAudioStream = *mut c_void;

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
