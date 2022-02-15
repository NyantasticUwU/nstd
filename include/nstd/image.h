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
    NSTD_IMAGE_FORMAT_UNKNOWN,
    NSTD_IMAGE_FORMAT_LUMA8,
    NSTD_IMAGE_FORMAT_LUMAA8,
    NSTD_IMAGE_FORMAT_RGB8,
    NSTD_IMAGE_FORMAT_RGBA8,
    NSTD_IMAGE_FORMAT_LUMA16,
    NSTD_IMAGE_FORMAT_LUMAA16,
    NSTD_IMAGE_FORMAT_RGB16,
    NSTD_IMAGE_FORMAT_RGBA16,
    NSTD_IMAGE_FORMAT_RGB32F,
    NSTD_IMAGE_FORMAT_RGBA32F
} NSTDImageFormat;

/// Represents an image.
typedef struct
{
    NSTDImageHandle image;
    const NSTDByte *raw;
    NSTDImageFormat format;
    NSTDUInt32 width;
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
