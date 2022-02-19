use crate::core::{def::NSTDChar, slice::NSTDSlice};
use image::{ColorType, DynamicImage as Image, GenericImageView};
use std::ffi::CStr;

/// Represents a pointer to some image data.
pub type NSTDImageHandle = *mut Image;

/// Represents an image format.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDImageFormat {
    /// An unknown image format.
    NSTD_IMAGE_FORMAT_UNKNOWN,
    /// The LUMA8 format.
    NSTD_IMAGE_FORMAT_LUMA8,
    /// The LUMAA8 format.
    NSTD_IMAGE_FORMAT_LUMAA8,
    /// The RGB8 format.
    NSTD_IMAGE_FORMAT_RGB8,
    /// The RGBA8 format.
    NSTD_IMAGE_FORMAT_RGBA8,
    /// The LUMA16 format.
    NSTD_IMAGE_FORMAT_LUMA16,
    /// The LUMAA16 format.
    NSTD_IMAGE_FORMAT_LUMAA16,
    /// The RGB16 format.
    NSTD_IMAGE_FORMAT_RGB16,
    /// The RGBA16 format.
    NSTD_IMAGE_FORMAT_RGBA16,
    /// The RGB32F format.
    NSTD_IMAGE_FORMAT_RGB32F,
    /// The RGBA32F format.
    NSTD_IMAGE_FORMAT_RGBA32F,
}
impl From<&Image> for NSTDImageFormat {
    #[inline]
    fn from(image: &Image) -> Self {
        match image.color() {
            ColorType::L8 => Self::NSTD_IMAGE_FORMAT_LUMA8,
            ColorType::La8 => Self::NSTD_IMAGE_FORMAT_LUMAA8,
            ColorType::Rgb8 => Self::NSTD_IMAGE_FORMAT_RGB8,
            ColorType::Rgba8 => Self::NSTD_IMAGE_FORMAT_RGBA8,
            ColorType::L16 => Self::NSTD_IMAGE_FORMAT_LUMA16,
            ColorType::La16 => Self::NSTD_IMAGE_FORMAT_LUMAA16,
            ColorType::Rgb16 => Self::NSTD_IMAGE_FORMAT_RGB16,
            ColorType::Rgba16 => Self::NSTD_IMAGE_FORMAT_RGBA16,
            ColorType::Rgb32F => Self::NSTD_IMAGE_FORMAT_RGB32F,
            ColorType::Rgba32F => Self::NSTD_IMAGE_FORMAT_RGBA32F,
            _ => Self::NSTD_IMAGE_FORMAT_UNKNOWN,
        }
    }
}

/// Represents an image.
#[repr(C)]
pub struct NSTDImage {
    /// A raw handle to the image.
    pub image: NSTDImageHandle,
    /// A raw pointer to the image data.
    pub raw: *const u8,
    /// The image format.
    pub format: NSTDImageFormat,
    /// The width of the image in pixels.
    pub width: u32,
    /// The height of the image in pixels.
    pub height: u32,
}
impl Default for NSTDImage {
    #[inline]
    fn default() -> Self {
        Self {
            image: std::ptr::null_mut(),
            raw: std::ptr::null_mut(),
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
///     `const NSTDChar *const file_name` - Path to the image file.
/// Returns: `NSTDImage image` - The image.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_image_open(file_name: *const NSTDChar) -> NSTDImage {
    if let Ok(file_name) = CStr::from_ptr(file_name).to_str() {
        if let Ok(image) = image::open(file_name) {
            return NSTDImage::from(image);
        }
    }
    NSTDImage::default()
}

/// Loads an image from memory.
/// Parameters:
///     `const NSTDSlice *const bytes` - Raw image data.
/// Returns: `NSTDImage image` - The image.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_image_load(bytes: &NSTDSlice) -> NSTDImage {
    if bytes.ptr.size == 1 {
        if let Ok(image) = image::load_from_memory(bytes.as_byte_slice()) {
            return NSTDImage::from(image);
        }
    }
    NSTDImage::default()
}

/// Frees image data.
/// Parameters:
///     `NSTDImage *const image` - Pointer to the image data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_image_free(image: &mut NSTDImage) {
    Box::from_raw(image.image);
    image.image = std::ptr::null_mut();
}
