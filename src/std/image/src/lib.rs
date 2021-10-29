use image::{DynamicImage as Image, GenericImageView};
use std::{ffi::CStr, os::raw::c_char, ptr};

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
        match *image {
            Image::ImageLuma8(_) => Self::NSTD_IMAGE_FORMAT_LUMA8,
            Image::ImageLumaA8(_) => Self::NSTD_IMAGE_FORMAT_LUMAA8,
            Image::ImageRgb8(_) => Self::NSTD_IMAGE_FORMAT_RGB8,
            Image::ImageRgba8(_) => Self::NSTD_IMAGE_FORMAT_RGBA8,
            Image::ImageBgr8(_) => Self::NSTD_IMAGE_FORMAT_BGR8,
            Image::ImageBgra8(_) => Self::NSTD_IMAGE_FORMAT_BGRA8,
            Image::ImageLuma16(_) => Self::NSTD_IMAGE_FORMAT_LUMA16,
            Image::ImageLumaA16(_) => Self::NSTD_IMAGE_FORMAT_LUMAA16,
            Image::ImageRgb16(_) => Self::NSTD_IMAGE_FORMAT_RGB16,
            Image::ImageRgba16(_) => Self::NSTD_IMAGE_FORMAT_RGBA16,
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
#[no_mangle]
pub unsafe extern "C" fn nstd_std_image_open(file_name: *const c_char) -> NSTDImage {
    match CStr::from_ptr(file_name).to_str() {
        Ok(file_name) => match image::open(file_name) {
            Ok(image) => NSTDImage::from(image),
            _ => NSTDImage::default(),
        },
        _ => NSTDImage::default(),
    }
}

/// Frees image data.
/// Parameters:
///     `NSTDImage *image` - Pointer to the image data.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_image_free(image: &mut NSTDImage) {
    Box::from_raw(image.image);
    image.image = ptr::null_mut();
}
