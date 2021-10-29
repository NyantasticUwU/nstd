use image::DynamicImage as Image;
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

/// Represents an image.
#[repr(C)]
pub struct NSTDImage {
    pub image: NSTDImageHandle,
    pub raw: *const u8,
    pub format: NSTDImageFormat,
}
impl Default for NSTDImage {
    #[inline]
    fn default() -> Self {
        Self {
            image: ptr::null_mut(),
            raw: ptr::null_mut(),
            format: NSTDImageFormat::NSTD_IMAGE_FORMAT_UNKNOWN,
        }
    }
}

/// Opens an image from a file.
/// Parameters:
///     `const char *const file_name` - Path to the image file.
/// Returns: `NSTDImage image` - The image.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_image_open(file_name: *const c_char) -> NSTDImage {
    match CStr::from_ptr(file_name).to_str() {
        Ok(file_name) => match image::open(file_name) {
            Ok(img) => {
                let format = match img {
                    Image::ImageLuma8(_) => NSTDImageFormat::NSTD_IMAGE_FORMAT_LUMA8,
                    Image::ImageLumaA8(_) => NSTDImageFormat::NSTD_IMAGE_FORMAT_LUMAA8,
                    Image::ImageRgb8(_) => NSTDImageFormat::NSTD_IMAGE_FORMAT_RGB8,
                    Image::ImageRgba8(_) => NSTDImageFormat::NSTD_IMAGE_FORMAT_RGBA8,
                    Image::ImageBgr8(_) => NSTDImageFormat::NSTD_IMAGE_FORMAT_BGR8,
                    Image::ImageBgra8(_) => NSTDImageFormat::NSTD_IMAGE_FORMAT_BGRA8,
                    Image::ImageLuma16(_) => NSTDImageFormat::NSTD_IMAGE_FORMAT_LUMA16,
                    Image::ImageLumaA16(_) => NSTDImageFormat::NSTD_IMAGE_FORMAT_LUMAA16,
                    Image::ImageRgb16(_) => NSTDImageFormat::NSTD_IMAGE_FORMAT_RGB16,
                    Image::ImageRgba16(_) => NSTDImageFormat::NSTD_IMAGE_FORMAT_RGBA16,
                };
                let image = Box::into_raw(Box::new(img));
                let raw = (*image).as_bytes().as_ptr();
                NSTDImage { image, raw, format }
            }
            _ => NSTDImage::default(),
        },
        _ => NSTDImage::default(),
    }
}

/// Generates nstd_std_image_get_(height | width) functions.
macro_rules! nstd_img_get_size {
    ($name: ident, $size_type: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(image: NSTDImage) -> u32 {
            match *image.image {
                Image::ImageLuma8(ref buf) => buf.$size_type(),
                Image::ImageLumaA8(ref buf) => buf.$size_type(),
                Image::ImageRgb8(ref buf) => buf.$size_type(),
                Image::ImageRgba8(ref buf) => buf.$size_type(),
                Image::ImageBgr8(ref buf) => buf.$size_type(),
                Image::ImageBgra8(ref buf) => buf.$size_type(),
                Image::ImageLuma16(ref buf) => buf.$size_type(),
                Image::ImageLumaA16(ref buf) => buf.$size_type(),
                Image::ImageRgb16(ref buf) => buf.$size_type(),
                Image::ImageRgba16(ref buf) => buf.$size_type(),
            }
        }
    };
}
nstd_img_get_size!(nstd_std_image_get_width, width);
nstd_img_get_size!(nstd_std_image_get_height, height);

/// Frees image data.
/// Parameters:
///     `NSTDImage *image` - Pointer to the image data.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_image_free(image: &mut NSTDImage) {
    Box::from_raw(image.image);
    image.image = ptr::null_mut();
}
