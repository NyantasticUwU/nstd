#ifndef NSTD_CORE_VIEW_H_INCLUDED
#define NSTD_CORE_VIEW_H_INCLUDED
#include "def.h"
#include "pointer.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a view into a sequence of data.
typedef struct
{
    /// The number of elements the slice contains.
    NSTDUSize size;
    /// A pointer to the data.
    NSTDPointer ptr;
} NSTDSlice;

/// Creates a new slice from raw data.
/// Parameters:
///     `const NSTDUSize size` - Number of elements to slice.
///     `const NSTDUSize element_size` - Size of each element.
///     `NSTDAny data` - Pointer to the raw data.
/// Returns: `NSTDSlice slice` - The new slice.
NSTDAPI NSTDSlice nstd_core_slice_new(
    const NSTDUSize size,
    const NSTDUSize element_size,
    NSTDAny data);

/// Gets a pointer to an element from a slice.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the slice's memory
/// gets reallocated or deallocated, so it is advised to create a copy of the element after
/// getting it.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDUSize pos` - The position of the element to get.
/// Returns: `NSTDAny element` - Pointer to the element.
NSTDAPI NSTDAny nstd_core_slice_get(const NSTDSlice *const slice, const NSTDUSize pos);

/// Gets the first element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_core_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `NSTDAny element` - Pointer to the first element.
NSTDAPI NSTDAny nstd_core_slice_first(const NSTDSlice *const slice);

/// Gets the last element in the slice.
/// NOTE: This function follows the same behaviour rules as `nstd_core_slice_get`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
/// Returns: `NSTDAny element` - Pointer to the last element.
NSTDAPI NSTDAny nstd_core_slice_last(const NSTDSlice *const slice);

/// Checks if two slices carry the same data.
/// Parameters:
///     `const NSTDSlice *const s1` - The first slice.
///     `const NSTDSlice *const s2` - The second slice.
/// Returns: `NSTDBool is_same` - True if the two slices carry the same data.
NSTDAPI NSTDBool nstd_core_slice_compare(const NSTDSlice *const s1, const NSTDSlice *const s2);

/// Checks if a slice contains `element`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `NSTDAny element` - The element to search for.
/// Returns: `NSTDBool is_in` - True if the slice contains `element`.
NSTDAPI NSTDBool nstd_core_slice_contains(const NSTDSlice *const slice, NSTDAny element);

/// Counts the number of `element`s found in `slice`.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `NSTDAny element` - The element to count.
/// Returns: `NSTDUSize count` - The number of `element`s in `slice`.
NSTDAPI NSTDUSize nstd_core_slice_count(const NSTDSlice *const slice, NSTDAny element);

/// Finds the first `element` in `slice` and returns the index of the element.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `NSTDAny element` - The element to search for.
/// Returns: `NSTDUSize index` - The index of the element, -1/usize::MAX if not found.
NSTDAPI NSTDUSize nstd_core_slice_find_first(const NSTDSlice *const slice, NSTDAny element);

/// Finds the last `element` in `slice` and returns the index of the element.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `NSTDAny element` - The element to search for.
/// Returns: `NSTDUSize index` - The index of the element, -1/usize::MAX if not found.
NSTDAPI NSTDUSize nstd_core_slice_find_last(const NSTDSlice *const slice, NSTDAny element);

/// Checks if a slice starts with another slice.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDSlice *const pattern` - The slice pattern.
/// Returns: `NSTDBool starts_with` - True if `slice` starts with `pattern`.
NSTDAPI NSTDBool nstd_core_slice_starts_with(
    const NSTDSlice *const slice,
    const NSTDSlice *const pattern);

/// Checks if a slice ends with another slice.
/// Parameters:
///     `const NSTDSlice *const slice` - The slice.
///     `const NSTDSlice *const pattern` - The slice pattern.
/// Returns: `NSTDBool ends_with` - True if `slice` ends with `pattern`.
NSTDAPI NSTDBool nstd_core_slice_ends_with(
    const NSTDSlice *const slice,
    const NSTDSlice *const pattern);

/// Fills a slice with `element`.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `NSTDAny element` - The element.
NSTDAPI void nstd_core_slice_fill(NSTDSlice *const slice, NSTDAny element);

/// Fills a specific range of a slice with `element`.
/// NOTE: This function does NOT check that `range` is valid for operating on `slice`.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `NSTDAny element` - The element.
///     `const NSTDURange *const range` - The range of the slice to fill.
NSTDAPI void nstd_core_slice_fill_range(
    NSTDSlice *const slice,
    NSTDAny element,
    const NSTDURange *const range);

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

/// Shifts a slice `x` times to the right.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDUSize x` - Number of times to shift the slice.
NSTDAPI void nstd_core_slice_shift_right(NSTDSlice *const slice, const NSTDUSize x);

/// Shifts a slice `x` times to the left.
/// Parameters:
///     `NSTDSlice *const slice` - The slice.
///     `const NSTDUSize x` - Number of times to shift the slice.
NSTDAPI void nstd_core_slice_shift_left(NSTDSlice *const slice, const NSTDUSize x);

/// Copies elements from `s2` to `s1`. The slices must be the same size in bytes.
/// Parameters:
///     `NSTDSlice *const s1` - The slice to copy to.
///     `const NSTDSlice *const s2` - The slice to copy from.
NSTDAPI void nstd_core_slice_copy_from_slice(NSTDSlice *const s1, const NSTDSlice *const s2);

/// Swaps the elements of `s1` and `s2`. The slices must be the same size in bytes.
/// Parameters:
///     `NSTDSlice *const s1` - The first slice.
///     `NSTDSlice *const s2` - The second slice.
NSTDAPI void nstd_core_slice_swap_with_slice(NSTDSlice *const s1, NSTDSlice *const s2);

/// Moves bytes from `s2` to `s1`, sets all `s2` bytes to 0.
/// Parameters:
///     `NSTDSlice *const s1` - The first slice.
///     `NSTDSlice *const s2` - The second slice.
NSTDAPI void nstd_core_slice_move(NSTDSlice *const s1, NSTDSlice *const s2);

#ifdef __cplusplus
}
#endif
#endif
