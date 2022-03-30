//! An audio I/O device handle.
use super::{
    def::{NSTDAudioSampleFormat, NSTDAudioStreamConfig},
    stream::NSTDAudioStream,
};
use crate::{
    core::def::{NSTDAny, NSTDAnyConst, NSTDErrorCode},
    string::NSTDString,
};
use cpal::{
    traits::*, BufferSize, BuildStreamError, Device, Sample, SampleFormat, SampleRate, Stream,
    StreamConfig,
};

/// Represents an audio device.
pub type NSTDAudioDevice = *mut Device;

/// Gets the name of a device.
///
/// # Parameters
///
/// - `const NSTDAudioDevice device` - The device.
///
/// # Returns
///
/// `NSTDString name` - The device name.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_device_name(device: NSTDAudioDevice) -> NSTDString {
    if let Ok(name) = (*device).name() {
        return NSTDString::from(name.as_bytes());
    }
    let null = crate::core::slice::nstd_core_slice_new(0, 0, std::ptr::null_mut());
    let null = crate::vec::nstd_vec_from_existing(0, &null);
    crate::string::nstd_string_from_existing(&null)
}

/// Generates `nstd_audio_device_default_*_config` functions.
macro_rules! generate_device_default_config {
    ($name: ident, $method: ident) => {
        ///
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(
            device: NSTDAudioDevice,
            config: *mut NSTDAudioStreamConfig,
        ) -> NSTDErrorCode {
            if let Ok(cpal_supported_config) = (*device).$method() {
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
                return 0;
            }
            1
        }
    };
}
generate_device_default_config!(nstd_audio_device_default_input_config, default_input_config);
generate_device_default_config!(
    nstd_audio_device_default_output_config,
    default_output_config
);

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
generate_build_stream!(build_input_stream, as_ptr, NSTDAnyConst, &[T]);
generate_build_stream!(build_output_stream, as_mut_ptr, NSTDAny, &mut [T]);

/// Generates `nstd_audio_device_build_*_stream` functions.
macro_rules! generate_device_build_stream {
    ($name: ident, $func: ident, $ptr_ty: ty) => {
        ///
        #[cfg_attr(feature = "clib", no_mangle)]
        pub unsafe extern "C" fn $name(
            device: NSTDAudioDevice,
            config: &NSTDAudioStreamConfig,
            callback: extern "C" fn($ptr_ty, usize),
            err_callback: extern "C" fn(),
        ) -> NSTDAudioStream {
            let stream_config = StreamConfig {
                channels: (*config).channels,
                sample_rate: SampleRate((*config).sample_rate),
                buffer_size: match (*config).buffer_size {
                    0 => BufferSize::Default,
                    size => BufferSize::Fixed(size),
                },
            };
            let stream = match config.format {
                NSTDAudioSampleFormat::INT16 => {
                    $func::<i16>(&*device, &stream_config, callback, err_callback)
                }
                NSTDAudioSampleFormat::UINT16 => {
                    $func::<u16>(&*device, &stream_config, callback, err_callback)
                }
                NSTDAudioSampleFormat::FLOAT32 => {
                    $func::<f32>(&*device, &stream_config, callback, err_callback)
                }
            };
            if let Ok(stream) = stream {
                if stream.play().is_ok() {
                    return Box::into_raw(Box::new(stream));
                }
            }
            std::ptr::null_mut()
        }
    };
}
generate_device_build_stream!(
    nstd_audio_device_build_input_stream,
    build_input_stream,
    NSTDAnyConst
);
generate_device_build_stream!(
    nstd_audio_device_build_output_stream,
    build_output_stream,
    NSTDAny
);

/// Frees a device.
///
/// # Parameters
///
/// - `NSTDAudioDevice *const device` - Pointer to a device.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_audio_device_free(device: *mut NSTDAudioDevice) {
    Box::from_raw(*device);
    *device = std::ptr::null_mut();
}
