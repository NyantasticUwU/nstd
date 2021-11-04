#ifndef NSTD_STD_COLLECTIONS_SLICE_H_INCLUDED
#define NSTD_STD_COLLECTIONS_SLICE_H_INCLUDED
#include "../../core/def.h"
#include "../def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a view into a sequence of data.
typedef struct
{
    NSTDSize size;
    NSTDSize element_size;
    NSTDByte *data;
} NSTDSlice;

/// Creates a new slice from raw data.
/// Parameters:
///     `const NSTDSize size` - Number of elements to view.
///     `const NSTDSize element_size` - Size of each element.
///     `NSTDByte *const data` - Pointer to the raw data.
/// Returns: `NSTDSlice slice` - The new slice.
NSTDAPI NSTDSlice nstd_std_collections_slice_new(
    const NSTDSize size,
    const NSTDSize element_size,
    NSTDByte *const data);

/// Gets a pointer to an element from a slice.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the slice's memory
/// gets reallocated or deallocated, so it is advised to create a copy of the element after
/// getting it.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDSize pos` - The position of the element to get.
/// Returns: `void *element` - Pointer to the element.
NSTDAPI void *nstd_std_collections_slice_get(const NSTDSlice *const slice, const NSTDSize pos);

/// Gets the first element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_std_collections_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `void *element` - Pointer to the first element.
NSTDAPI void *nstd_std_collections_slice_first(const NSTDSlice *const slice);

/// Gets the last element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_std_collections_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `void *element` - Pointer to the last element.
NSTDAPI void *nstd_std_collections_slice_last(const NSTDSlice *const slice);

/// Reverses a slice's elements.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
NSTDAPI void nstd_std_collections_slice_reverse(const NSTDSlice *const slice);

#ifdef __cplusplus
}
#endif
#endif
