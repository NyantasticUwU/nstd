use crate::{
    core::def::{NSTDAny, NSTDAnyConst, NSTDErrorCode},
    vec::*,
};

/// A stack collection type.
#[repr(C)]
pub struct NSTDStack {
    /// The underlying vector.
    pub buffer: NSTDVec,
}

/// Creates a new stack.
/// Parameters:
///     `const NSTDUSize element_size` - The size of each element on the stack.
/// Returns: `NSTDStack stack` - The new stack.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_stack_new(element_size: usize) -> NSTDStack {
    NSTDStack {
        buffer: nstd_vec_new(element_size),
    }
}

/// Gets a pointer to the value at the top of the stack.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the stack's memory.
/// Parameters:
///     `const NSTDStack *const stack` - A pointer to the stack.
/// Returns: `NSTDAny top` - The value at the top of the stack.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_stack_top(stack: &NSTDStack) -> NSTDAny {
    nstd_vec_last(&stack.buffer)
}

/// Pushes an element onto the stack.
/// Parameters:
///     `NSTDStack *const stack` - Pointer to the stack.
///     `const NSTDAnyConst element` - A pointer to the element to push onto the stack.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_stack_push(
    stack: &mut NSTDStack,
    element: NSTDAnyConst,
) -> NSTDErrorCode {
    nstd_vec_push(&mut stack.buffer, element)
}

/// Pops a value off of the stack and returns a pointer to it.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the stack's memory.
/// Parameters:
///     `NSTDStack *const stack` - Pointer to the stack.
/// Returns: `NSTDAny popped` - The value that was popped off the stack.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_stack_pop(stack: &mut NSTDStack) -> NSTDAny {
    nstd_vec_pop(&mut stack.buffer)
}

/// Pops all values off the stack.
/// Parameters:
///     `NSTDStack *const stack` - Pointer to the stack.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_stack_clear(stack: &mut NSTDStack) {
    nstd_vec_clear(&mut stack.buffer);
}

/// Returns the number of elements on the stack.
/// Parameters:
///     `const NSTDStack *const stack` - Pointer to the stack.
/// Returns: `NSTDUSize len` - The length of the stack.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_stack_len(stack: &NSTDStack) -> usize {
    stack.buffer.size
}

/// Frees stack data.
/// Parameters:
///     `NSTDStack *const stack` - A pointer to the stack to free.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_stack_free(stack: &mut NSTDStack) -> NSTDErrorCode {
    nstd_vec_free(&mut stack.buffer)
}
