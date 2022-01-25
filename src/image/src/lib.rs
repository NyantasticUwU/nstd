use image::{ColorType, DynamicImage as Image, GenericImageView};
use nstd_core::slice::NSTDSlice;
use std::{ffi::CStr, os::raw::c_char, ptr};
#[cfg(feature = "deps")]
pub mod deps {
    pub use image;
    pub use nstd_core;
}

/// Represents a pointer to some image data.
pub type NSTDImageHandle = *mut Image;

/// Represents an image format.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDImageFormat {
    NSTD_IMAGE_FORMAT_UNKNOWN,
    NSTD_IMAGE_FORMAT_LUMA8,
    NSTD_IMAGE_FORMAT_LUMAA8,
    NSTD_IMAGE_FORMAT_RGB8,
    NSTD_IMAGE_FORMAT_RGBA8,
    NSTD_IMAGE_FORMAT_BGR8,
    NSTD_IMAGE_FORMAT_BGRA8,
    NSTD_IMAGE_FORMAT_LUMA16,
    NSTD_IMAGE_FORMAT_LUMAA16,
    NSTD_IMAGE_FORMAT_RGB16,
    NSTD_IMAGE_FORMAT_RGBA16,
}
impl From<&Image> for NSTDImageFormat {
    #[inline]
    fn from(image: &Image) -> Self {
        match image.color() {
            ColorType::L8 => Self::NSTD_IMAGE_FORMAT_LUMA8,
            ColorType::La8 => Self::NSTD_IMAGE_FORMAT_LUMAA8,
            ColorType::Rgb8 => Self::NSTD_IMAGE_FORMAT_RGB8,
            ColorType::Rgba8 => Self::NSTD_IMAGE_FORMAT_RGBA8,
            ColorType::Bgr8 => Self::NSTD_IMAGE_FORMAT_BGR8,
            ColorType::Bgra8 => Self::NSTD_IMAGE_FORMAT_BGRA8,
            ColorType::L16 => Self::NSTD_IMAGE_FORMAT_LUMA16,
            ColorType::La16 => Self::NSTD_IMAGE_FORMAT_LUMAA16,
            ColorType::Rgb16 => Self::NSTD_IMAGE_FORMAT_RGB16,
            ColorType::Rgba16 => Self::NSTD_IMAGE_FORMAT_RGBA16,
            _ => Self::NSTD_IMAGE_FORMAT_UNKNOWN,
        }
    }
}

/// Represents an image.
#[repr(C)]
pub struct NSTDImage {
    pub image: NSTDImageHandle,
    pub raw: *const u8,
    pub format: NSTDImageFormat,
    pub width: u32,
    pub height: u32,
}
impl Default for NSTDImage {
    #[inline]
    fn default() -> Self {
        Self {
            image: ptr::null_mut(),
            raw: ptr::null_mut(),
            format: NSTDImageFormat::NSTD_IMAGE_FORMAT_UNKNOWN,
            width: 0,
            height: 0,
        }
    }
}
impl From<Image> for NSTDImage {
    #[inline]
    fn from(image: Image) -> Self {
        unsafe {
            let image = Box::into_raw(Box::new(image));
            let raw = (*image).as_bytes().as_ptr();
            let format = NSTDImageFormat::from(&*image);
            let (width, height) = (*image).dimensions();
            Self {
                image,
                raw,
                format,
                width,
                height,
            }
        }
    }
}

/// Opens an image from a file.
/// Parameters:
///     `const char *const file_name` - Path to the image file.
/// Returns: `NSTDImage image` - The image.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_image_open(file_name: *const c_char) -> NSTDImage {
    match CStr::from_ptr(file_name).to_str() {
        Ok(file_name) => match image::open(file_name) {
            Ok(image) => NSTDImage::from(image),
            _ => NSTDImage::default(),
        },
        _ => NSTDImage::default(),
    }
}

/// Loads an image from memory.
/// Parameters:
///     `const NSTDSlice *const bytes` - Raw image data.
/// Returns: `NSTDImage image` - The image.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_image_load(bytes: &NSTDSlice) -> NSTDImage {
    if bytes.ptr.size == 1 {
        match image::load_from_memory(bytes.as_byte_slice()) {
            Ok(image) => return NSTDImage::from(image),
            _ => (),
        }
    }
    NSTDImage::default()
}

/// Frees image data.
/// Parameters:
///     `NSTDImage *image` - Pointer to the image data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_image_free(image: &mut NSTDImage) {
    Box::from_raw(image.image);
    image.image = ptr::null_mut();
}
