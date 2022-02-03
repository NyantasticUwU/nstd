#ifndef NSTD_COLLECTIONS_VEC_H_INCLUDED
#define NSTD_COLLECTIONS_VEC_H_INCLUDED
#include "../core/def.h"
#include "../core/slice.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents an array of dynamic length.
typedef struct
{
    /// The number of active elements in this vector.
    NSTDUSize size;
    /// Buffer of allocated memory where `buffer.size` is the capacity, `buffer.ptr.size` is the
    /// size of each element, and `buffer.ptr.raw` is a raw pointer to the buffer.
    NSTDSlice buffer;
} NSTDVec;

/// Creates a new vector.
/// Parameters:
///     `const NSTDUSize element_size` - The size of each element in the vector.
/// Returns: `NSTDVec vec` - The new vector.
NSTDAPI NSTDVec nstd_collections_vec_new(const NSTDUSize element_size);

/// Creates a new vector with the specified capacity.
/// Parameters:
///     `const NSTDUSize element_size` - The size of each element in the vector.
///     `const NSTDUSize capacity` - The capacity to give the vector, must be greater than 0.
/// Returns: `NSTDVec vec` - The new vector.
NSTDAPI NSTDVec nstd_collections_vec_new_with_capacity(
    const NSTDUSize element_size,
    const NSTDUSize capacity);

/// Creates an `NSTDSlice` from an `NSTDVec`.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
/// Returns: `NSTDSlice slice` - The new slice.
NSTDAPI NSTDSlice nstd_collections_vec_as_slice(const NSTDVec *const vec);

/// Gets a pointer to an element from a vector.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the vector's memory
/// gets reallocated or deallocated, so it is advised to create a copy of the element after
/// getting it.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
///     `const NSTDUSize pos` - The position of the element to get.
/// Returns: `NSTDAny element` - Pointer to the element.
NSTDAPI NSTDAny nstd_collections_vec_get(const NSTDVec *const vec, const NSTDUSize pos);

/// Gets the first element in the vector.
/// NOTE: This function follows the same behaviour rules as `nstd_collections_vec_get`.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
/// Returns: `NSTDAny element` - Pointer to the first element.
NSTDAPI NSTDAny nstd_collections_vec_first(const NSTDVec *const vec);

/// Gets the last element in the vector.
/// NOTE: This function follows the same behaviour rules as `nstd_collections_vec_get`.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
/// Returns: `NSTDAny element` - Pointer to the last element.
NSTDAPI NSTDAny nstd_collections_vec_last(const NSTDVec *const vec);

/// Pushes a new element onto the end of a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDAnyConst element` - Pointer to the new element.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_collections_vec_push(NSTDVec *const vec, const NSTDAnyConst element);

/// Pops a value off of the back of a vector and returns a pointer to it.
/// NOTE: This function follows the same behaviour rules as `nstd_collections_vec_get`.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
/// Returns: `NSTDAny element` - The element that was removed.
NSTDAPI NSTDAny nstd_collections_vec_pop(NSTDVec *const vec);

/// Extends a vector from a slice. `vec` and `slice` must have the same element size.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDSlice *const slice` - The slice to extend from.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_collections_vec_extend(NSTDVec *const vec, const NSTDSlice *const slice);

/// Inserts an element at `index` for a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDAnyConst element` - Pointer to the new element.
///     `const NSTDUSize index` - The index to insert an element.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_collections_vec_insert(
    NSTDVec *const vec,
    const NSTDAnyConst element,
    const NSTDUSize index);

/// Removes an element at `index` for a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDUSize index` - The index of the element to remove.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_collections_vec_remove(NSTDVec *const vec, const NSTDUSize index);

/// Clears the contents of a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
NSTDAPI void nstd_collections_vec_clear(NSTDVec *const vec);

/// Resizes a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDUSize new_size` - The new vector size.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_collections_vec_resize(NSTDVec *const vec, const NSTDUSize new_size);

/// Reserves memory for the vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDUSize new_cap` - The new, greater capacity for the vector.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_collections_vec_reserve(NSTDVec *const vec, const NSTDUSize new_cap);

/// Shrinks a vector to free any unused memory.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_collections_vec_shrink(NSTDVec *const vec);

/// Frees a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_collections_vec_free(NSTDVec *const vec);

#ifdef __cplusplus
}
#endif
#endif
