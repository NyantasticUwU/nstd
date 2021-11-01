#ifndef NSTD_STD_COLLECTIONS_VEC_H_INCLUDED
#define NSTD_STD_COLLECTIONS_VEC_H_INCLUDED
#include "../../core/def.h"
#include "../def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents an array of dynamic length.
typedef struct
{
    NSTDSize size;
    NSTDSize capacity;
    NSTDSize element_size;
    NSTDByte *data;
} NSTDVec;

/// Creates a new vector.
/// Parameters:
///     `const NSTDSize element_size` - The size of each element in the vector.
/// Returns: `NSTDVec vec` - The new vector.
NSTDAPI NSTDVec nstd_std_collections_vec_new(const NSTDSize element_size);

/// Creates a new vector with the specified capacity.
/// Parameters:
///     `const NSTDSize element_size` - The size of each element in the vector.
///     `const NSTDSize capacity` - The capacity to give the vector, must be greater than 0.
/// Returns: `NSTDVec vec` - The new vector.
NSTDAPI NSTDVec nstd_std_collections_vec_new_with_capacity(
    const NSTDSize element_size,
    const NSTDSize capacity);

/// Gets a pointer to an element from a vector.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the vector's memory
/// gets reallocated or deallocated, so it is advised to create a copy of the element after
/// getting it.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
///     `const NSTDSize pos` - The position of the element to get.
/// Returns: `void *element` - Pointer to the element.
NSTDAPI void *nstd_std_collections_vec_get(const NSTDVec *const vec, const NSTDSize pos);

/// Gets the first element in the vector.
/// NOTE: This function follows the same behaviour rules as `nstd_std_collections_vec_get`.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
/// Returns: `void *element` - Pointer to the first element.
NSTDAPI void *nstd_std_collections_vec_first(const NSTDVec *const vec);

/// Gets the last element in the vector.
/// NOTE: This function follows the same behaviour rules as `nstd_std_collections_vec_get`.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
/// Returns: `void *element` - Pointer to the last element.
NSTDAPI void *nstd_std_collections_vec_last(const NSTDVec *const vec);

/// Pushes a new element onto the end of a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const void *const element` - Pointer to the new element.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_collections_vec_push(NSTDVec *const vec, const void *const element);

/// Pops a value off of the back of a vector and returns a pointer to it.
/// NOTE: This function follows the same behaviour rules as `nstd_std_collections_vec_get`.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
/// Returns: `void *element` - The element that was removed.
NSTDAPI void *nstd_std_collections_vec_pop(NSTDVec *const vec);

/// Inserts an element at `index` for a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const void *const element` - Pointer to the new element.
///     `const NSTDSize index` - The index to insert an element.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_collections_vec_insert(
    NSTDVec *const vec,
    const void *const element,
    const NSTDSize index);

/// Removes an element at `index` for a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDSize index` - The index of the element to remove.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_collections_vec_remove(NSTDVec *const vec, const NSTDSize index);

/// Clears the contents of a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
NSTDAPI void nstd_std_collections_vec_clear(NSTDVec *const vec);

/// Resizes a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDSize new_size` - The new vector size.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_collections_vec_resize(NSTDVec *const vec, const NSTDSize new_size);

/// Reserves memory for the vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDSize new_cap` - The new, greater capacity for the vector.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_collections_vec_reserve(NSTDVec *const vec, const NSTDSize new_cap);

/// Shrinks a vector to free any unused memory.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_collections_vec_shrink(NSTDVec *const vec);

/// Frees a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_collections_vec_free(NSTDVec *const vec);

#ifdef __cplusplus
}
#endif
#endif
