#ifndef NSTD_CORE_VIEW_H_INCLUDED
#define NSTD_CORE_VIEW_H_INCLUDED
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a view into a sequence of data.
typedef struct
{
    NSTDUSize size;
    NSTDUSize element_size;
    NSTDByte *data;
} NSTDSlice;

/// Creates a new slice from raw data.
/// Parameters:
///     `const NSTDUSize size` - Number of elements to slice.
///     `const NSTDUSize element_size` - Size of each element.
///     `NSTDByte *const data` - Pointer to the raw data.
/// Returns: `NSTDSlice slice` - The new slice.
NSTDAPI NSTDSlice nstd_core_slice_new(
    const NSTDUSize size,
    const NSTDUSize element_size,
    NSTDByte *const data);

/// Gets a pointer to an element from a slice.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the slice's memory
/// gets reallocated or deallocated, so it is advised to create a copy of the element after
/// getting it.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDUSize pos` - The position of the element to get.
/// Returns: `void *element` - Pointer to the element.
NSTDAPI void *nstd_core_slice_get(const NSTDSlice *const slice, const NSTDUSize pos);

/// Gets the first element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_core_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `void *element` - Pointer to the first element.
NSTDAPI void *nstd_core_slice_first(const NSTDSlice *const slice);

/// Gets the last element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_core_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `void *element` - Pointer to the last element.
NSTDAPI void *nstd_core_slice_last(const NSTDSlice *const slice);

/// Checks if a slice contains `element`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const void *const element` - The element to search for.
/// Returns: `NSTDInt32 is_in` - Nonzero if the slice contains `element`.
NSTDAPI NSTDInt32 nstd_core_slice_contains(const NSTDSlice *const slice, const void *const element);

/// Checks if a slice starts with another slice.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDSlice *const pattern` - The slice pattern.
/// Returns: `NSTDInt32 starts_with` - Nonzero if `slice` starts with `pattern`.
NSTDAPI NSTDInt32 nstd_core_slice_starts_with(
    const NSTDSlice *const slice,
    const NSTDSlice *const pattern);

/// Checks if a slice ends with another slice.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDSlice *const pattern` - The slice pattern.
/// Returns: `NSTDInt32 ends_with` - Nonzero if `slice` ends with `pattern`.
NSTDAPI NSTDInt32 nstd_core_slice_ends_with(
    const NSTDSlice *const slice,
    const NSTDSlice *const pattern);

/// Swaps two elements in a slice.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDUSize i` - The index of the first element.
///     `const NSTDUSize j` - The index of the second element.
NSTDAPI void nstd_core_slice_swap(NSTDSlice *const slice, const NSTDUSize i, const NSTDUSize j);

/// Reverses a slice's elements.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
NSTDAPI void nstd_core_slice_reverse(NSTDSlice *const slice);

#ifdef __cplusplus
}
#endif
#endif
