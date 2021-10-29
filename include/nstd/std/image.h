#ifndef NSTD_STD_IMAGE_H_INCLUDED
#define NSTD_STD_IMAGE_H_INCLUDED
#include "../core/def.h"
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a pointer to some image data.
typedef void *NSTDImageHandle;

/// Represents an image format.
typedef enum
{
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
    NSTD_IMAGE_FORMAT_RGBA16
} NSTDImageFormat;

/// Represents an image.
typedef struct
{
    NSTDImageHandle image;
    const NSTDByte *raw;
    NSTDImageFormat format;
} NSTDImage;

/// Opens an image from a file.
/// Parameters:
///     `const char *const file_name` - Path to the image file.
/// Returns: `NSTDImage image` - The image.
NSTDAPI NSTDImage nstd_std_image_open(const char *const file_name);

/// Gets the width of an image.
/// Parameters:
///     `NSTDImage image` - The image.
/// Returns: `NSTDUInt32 width` - The width of the image.
NSTDAPI NSTDUInt32 nstd_std_image_get_width(NSTDImage image);

/// Gets the height of an image.
/// Parameters:
///     `NSTDImage image` - The image.
/// Returns: `NSTDUInt32 height` - The height of the image.
NSTDAPI NSTDUInt32 nstd_std_image_get_height(NSTDImage image);

/// Frees image data.
/// Parameters:
///     `NSTDImage *image` - Pointer to the image data.
NSTDAPI void nstd_std_image_free(NSTDImage *image);

#ifdef __cplusplus
}
#endif
#endif
