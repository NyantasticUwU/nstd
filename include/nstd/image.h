#ifndef NSTD_IMAGE_H_INCLUDED
#define NSTD_IMAGE_H_INCLUDED
#include "core/def.h"
#include "core/slice.h"
#include "nstd.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a pointer to some image data.
typedef NSTDAny NSTDImageHandle;

/// Represents an image format.
typedef enum
{
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
    NSTD_IMAGE_FORMAT_RGBA32F
} NSTDImageFormat;

/// Represents an image.
typedef struct
{
    /// A raw handle to the image.
    NSTDImageHandle image;
    /// A raw pointer to the image data.
    const NSTDByte *raw;
    /// The image format.
    NSTDImageFormat format;
    /// The width of the image in pixels.
    NSTDUInt32 width;
    /// The height of the image in pixels.
    NSTDUInt32 height;
} NSTDImage;

/// Opens an image from a file.
/// Parameters:
///     `const NSTDChar *const file_name` - Path to the image file.
/// Returns: `NSTDImage image` - The image.
NSTDAPI NSTDImage nstd_image_open(const NSTDChar *const file_name);

/// Loads an image from memory.
/// Parameters:
///     `const NSTDSlice *const bytes` - Raw image data.
/// Returns: `NSTDImage image` - The image.
NSTDAPI NSTDImage nstd_image_load(const NSTDSlice *const bytes);

/// Frees image data.
/// Parameters:
///     `NSTDImage *const image` - Pointer to the image data.
NSTDAPI void nstd_image_free(NSTDImage *const image);

#ifdef __cplusplus
}
#endif
#endif
