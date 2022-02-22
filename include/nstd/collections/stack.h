#ifndef NSTD_COLLECTIONS_STACK_H_INCLUDED
#define NSTD_COLLECTIONS_STACK_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
#include "vec.h"
NSTDCPPSTART

/// A stack collection type.
typedef struct
{
    /// The underlying vector.
    NSTDVec buffer;
} NSTDStack;

/// Creates a new stack.
/// Parameters:
///     `const NSTDUSize element_size` - The size of each element on the stack.
/// Returns: `NSTDStack stack` - The new stack.
NSTDAPI NSTDStack nstd_collections_stack_new(const NSTDUSize element_size);

/// Gets a pointer to the value at the top of the stack.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the stack's memory.
/// Parameters:
///     `const NSTDStack *const stack` - A pointer to the stack.
/// Returns: `NSTDAny top` - The value at the top of the stack.
NSTDAPI NSTDAny nstd_collections_stack_top(const NSTDStack *const stack);

/// Pushes an element onto the stack.
/// Parameters:
///     `NSTDStack *const stack` - Pointer to the stack.
///     `const NSTDAnyConst element` - A pointer to the element to push onto the stack.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_collections_stack_push(
    NSTDStack *const stack,
    const NSTDAnyConst element);

/// Pops a value off of the stack and returns a pointer to it.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the stack's memory.
/// Parameters:
///     `NSTDStack *const stack` - Pointer to the stack.
/// Returns: `NSTDAny popped` - The value that was popped off the stack.
NSTDAPI NSTDAny nstd_collections_stack_pop(NSTDStack *const stack);

/// Pops all values off the stack.
/// Parameters:
///     `NSTDStack *const stack` - Pointer to the stack.
NSTDAPI void nstd_collections_stack_clear(NSTDStack *const stack);

/// Returns the number of elements on the stack.
/// Parameters:
///     `const NSTDStack *const stack` - Pointer to the stack.
/// Returns: `NSTDUSize len` - The length of the stack.
NSTDAPI NSTDUSize nstd_collections_stack_len(const NSTDStack *const stack);

/// Frees stack data.
/// Parameters:
///     `NSTDStack *const stack` - A pointer to the stack to free.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_collections_stack_free(NSTDStack *const stack);

NSTDCPPEND
#endif
