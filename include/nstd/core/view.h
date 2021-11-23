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
} NSTDView;

/// Creates a new view from raw data.
/// Parameters:
///     `const NSTDUSize size` - Number of elements to view.
///     `const NSTDUSize element_size` - Size of each element.
///     `NSTDByte *const data` - Pointer to the raw data.
/// Returns: `NSTDView view` - The new view.
NSTDAPI NSTDView nstd_core_view_new(
    const NSTDUSize size,
    const NSTDUSize element_size,
    NSTDByte *const data);

/// Gets a pointer to an element from a view.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the view's memory
/// gets reallocated or deallocated, so it is advised to create a copy of the element after
/// getting it.
/// Parameters:
///     `const NSTDView *const view` - The view.
///     `const NSTDUSize pos` - The position of the element to get.
/// Returns: `void *element` - Pointer to the element.
NSTDAPI void *nstd_core_view_get(const NSTDView *const view, const NSTDUSize pos);

/// Gets the first element in the view.
/// NOTE: This function follows the same behaviour rules as `nstd_core_view_get`.
/// Parameters:
///     `const NSTDView *const view` - The view.
/// Returns: `void *element` - Pointer to the first element.
NSTDAPI void *nstd_core_view_first(const NSTDView *const view);

/// Gets the last element in the view.
/// NOTE: This function follows the same behaviour rules as `nstd_core_view_get`.
/// Parameters:
///     `const NSTDView *const view` - The view.
/// Returns: `void *element` - Pointer to the last element.
NSTDAPI void *nstd_core_view_last(const NSTDView *const view);

/// Reverses a view's elements.
/// Parameters:
///     `const NSTDView *const view` - The view.
NSTDAPI void nstd_core_view_reverse(const NSTDView *const view);

#ifdef __cplusplus
}
#endif
#endif
