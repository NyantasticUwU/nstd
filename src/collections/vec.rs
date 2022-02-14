use crate::core::{
    def::{NSTDAny, NSTDAnyConst, NSTDErrorCode},
    slice::NSTDSlice,
};
use std::ptr::{self, addr_of};

/// Represents an array of dynamic length.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NSTDVec {
    /// The number of active elements in this vector.
    pub size: usize,
    /// Buffer of allocated memory where `buffer.size` is the capacity, `buffer.ptr.size` is the
    /// size of each element, and `buffer.ptr.raw` is a raw pointer to the buffer.
    pub buffer: NSTDSlice,
}
impl NSTDVec {
    /// Gets the total number of bytes allocated for this vec.
    #[inline]
    pub fn total_byte_count(&self) -> usize {
        self.buffer.size * self.buffer.ptr.size
    }

    /// Gets the number of used bytes for this vec.
    #[inline]
    pub fn byte_count(&self) -> usize {
        self.size * self.buffer.ptr.size
    }

    /// Returns a pointer to the end of the vector.
    #[inline]
    pub unsafe fn end_unchecked(&self) -> *mut u8 {
        self.buffer.ptr.raw.add(self.byte_count()).cast()
    }
}
impl Default for NSTDVec {
    #[inline]
    fn default() -> Self {
        unsafe {
            Self {
                size: 0,
                buffer: crate::core::slice::nstd_core_slice_new(0, 0, ptr::null_mut()),
            }
        }
    }
}
impl Clone for NSTDVec {
    fn clone(&self) -> Self {
        unsafe {
            let mut new_vec =
                nstd_collections_vec_new_with_capacity(self.buffer.ptr.size, self.buffer.size);
            if !new_vec.buffer.ptr.raw.is_null() {
                let byte_count = self.byte_count();
                let data = std::slice::from_raw_parts(self.buffer.ptr.raw as *const u8, byte_count);
                let new_data =
                    std::slice::from_raw_parts_mut(new_vec.buffer.ptr.raw.cast(), byte_count);
                new_data.copy_from_slice(data);
                new_vec.size = self.size;
            }
            new_vec
        }
    }
}
impl<T> From<Vec<T>> for NSTDVec {
    /// Creates an `NSTDVec` from a [`Vec`], forgets all values moved from `vec`.
    fn from(vec: Vec<T>) -> Self {
        unsafe {
            let element_size = std::mem::size_of::<T>();
            let mut nstd_vec = nstd_collections_vec_new_with_capacity(element_size, vec.len());
            if !nstd_vec.buffer.ptr.raw.is_null() {
                for element in vec {
                    nstd_collections_vec_push(&mut nstd_vec, addr_of!(element).cast());
                    std::mem::forget(element);
                }
            }
            nstd_vec
        }
    }
}

/// Creates a new vector.
/// Parameters:
///     `const NSTDUSize element_size` - The size of each element in the vector.
/// Returns: `NSTDVec vec` - The new vector.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_new(element_size: usize) -> NSTDVec {
    const INITIAL_CAPACITY: usize = 1;
    let data = crate::alloc::nstd_alloc_allocate(INITIAL_CAPACITY * element_size);
    NSTDVec {
        size: 0,
        buffer: crate::core::slice::nstd_core_slice_new(INITIAL_CAPACITY, element_size, data),
    }
}

/// Creates a new vector with the specified capacity.
/// Parameters:
///     `const NSTDUSize element_size` - The size of each element in the vector.
///     `const NSTDUSize capacity` - The capacity to give the vector, must be greater than 0.
/// Returns: `NSTDVec vec` - The new vector.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_new_with_capacity(
    element_size: usize,
    capacity: usize,
) -> NSTDVec {
    let data = crate::alloc::nstd_alloc_allocate(capacity * element_size);
    NSTDVec {
        size: 0,
        buffer: crate::core::slice::nstd_core_slice_new(capacity, element_size, data),
    }
}

/// Creates an `NSTDVec` object from existing data.
/// Parameters:
///     `const NSTDUSize size` - The number of active elements in the vector.
///     `const NSTDSlice *const buffer` - A slice of the whole data buffer.
/// Returns: `NSTDVec vec` - The new `NSTDVec` object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_from_existing(
    size: usize,
    buffer: &NSTDSlice,
) -> NSTDVec {
    NSTDVec {
        size,
        buffer: *buffer,
    }
}

/// Creates an `NSTDSlice` from an `NSTDVec`.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
/// Returns: `NSTDSlice slice` - The new slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_as_slice(vec: &NSTDVec) -> NSTDSlice {
    crate::core::slice::nstd_core_slice_new(vec.size, vec.buffer.ptr.size, vec.buffer.ptr.raw)
}

/// Gets a pointer to an element from a vector.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the vector's memory
/// gets reallocated or deallocated, so it is advised to create a copy of the element after
/// getting it.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
///     `const NSTDUSize pos` - The position of the element to get.
/// Returns: `NSTDAny element` - Pointer to the element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_get(vec: &NSTDVec, pos: usize) -> NSTDAny {
    match vec.size > pos {
        true => vec.buffer.ptr.raw.add(pos * vec.buffer.ptr.size),
        false => ptr::null_mut(),
    }
}

/// Gets the first element in the vector.
/// NOTE: This function follows the same behaviour rules as `nstd_collections_vec_get`.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
/// Returns: `NSTDAny element` - Pointer to the first element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_first(vec: &NSTDVec) -> NSTDAny {
    match vec.size > 0 {
        true => vec.buffer.ptr.raw,
        false => ptr::null_mut(),
    }
}

/// Gets the last element in the vector.
/// NOTE: This function follows the same behaviour rules as `nstd_collections_vec_get`.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
/// Returns: `NSTDAny element` - Pointer to the last element.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_last(vec: &NSTDVec) -> NSTDAny {
    match vec.size > 0 {
        true => vec.end_unchecked().sub(vec.buffer.ptr.size).cast(),
        false => ptr::null_mut(),
    }
}

/// Pushes a new element onto the end of a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDAnyConst element` - Pointer to the new element.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_push(
    vec: &mut NSTDVec,
    element: NSTDAnyConst,
) -> NSTDErrorCode {
    // Checking if the vector has reached it's capacity.
    if vec.size == vec.buffer.size {
        let new_cap = (vec.buffer.size as f32 * 1.5).ceil() as usize;
        match nstd_collections_vec_reserve(vec, new_cap) {
            0 => (),
            errc => return errc,
        }
        vec.buffer.size = new_cap;
    }
    // Adding new element.
    let element = std::slice::from_raw_parts(element as *const u8, vec.buffer.ptr.size);
    let data = std::slice::from_raw_parts_mut(vec.end_unchecked(), vec.buffer.ptr.size);
    data.copy_from_slice(element);
    vec.size += 1;
    0
}

/// Pops a value off of the back of a vector and returns a pointer to it.
/// NOTE: This function follows the same behaviour rules as `nstd_collections_vec_get`.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
/// Returns: `NSTDAny element` - The element that was removed.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_pop(vec: &mut NSTDVec) -> NSTDAny {
    match vec.size > 0 {
        true => {
            vec.size -= 1;
            vec.end_unchecked().cast()
        }
        false => ptr::null_mut(),
    }
}

/// Extends a vector from a slice. `vec` and `slice` must have the same element size.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDSlice *const slice` - The slice to extend from.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_extend(
    vec: &mut NSTDVec,
    slice: &NSTDSlice,
) -> NSTDErrorCode {
    if vec.buffer.ptr.size == slice.ptr.size {
        if slice.size > 0 {
            nstd_collections_vec_reserve(vec, vec.size + slice.size);
            let mut ptr = slice.ptr.raw;
            for _ in 0..slice.size {
                nstd_collections_vec_push(vec, ptr);
                ptr = ptr.add(slice.ptr.size);
            }
        }
        return 0;
    }
    1
}

/// Inserts an element at `index` for a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDAnyConst element` - Pointer to the new element.
///     `const NSTDUSize index` - The index to insert an element.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_insert(
    vec: &mut NSTDVec,
    element: NSTDAnyConst,
    index: usize,
) -> NSTDErrorCode {
    // A value is being inserted.
    if vec.size > index {
        // Checking if the vector has reached it's capacity.
        if vec.size == vec.buffer.size {
            match nstd_collections_vec_reserve(vec, vec.buffer.size + 1) {
                0 => (),
                errc => return errc,
            }
        }
        // Moving data up by one element.
        let index_pointer = nstd_collections_vec_get(vec, index) as *mut u8;
        let next_index_pointer = index_pointer.add(vec.buffer.ptr.size);
        let copy_size = (vec.size - index) * vec.buffer.ptr.size;
        ptr::copy(index_pointer, next_index_pointer, copy_size);
        // Inserting data.
        let element = std::slice::from_raw_parts(element as *const u8, vec.buffer.ptr.size);
        let data = std::slice::from_raw_parts_mut(index_pointer, vec.buffer.ptr.size);
        data.copy_from_slice(element);
        vec.size += 1;
        0
    }
    // A value is being pushed.
    else if vec.size == index {
        nstd_collections_vec_push(vec, element)
    }
    // Invalid index.
    else {
        1
    }
}

/// Removes an element at `index` for a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDUSize index` - The index of the element to remove.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_remove(
    vec: &mut NSTDVec,
    index: usize,
) -> NSTDErrorCode {
    match vec.size > index {
        true => {
            // Moving data down by one element.
            let index_pointer = nstd_collections_vec_get(vec, index) as *mut u8;
            let next_index_pointer = index_pointer.add(vec.buffer.ptr.size);
            let copy_size = (vec.size - index - 1) * vec.buffer.ptr.size;
            ptr::copy(next_index_pointer, index_pointer, copy_size);
            vec.size -= 1;
            0
        }
        false => 1,
    }
}

/// Clears the contents of a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_clear(vec: &mut NSTDVec) {
    vec.size = 0;
}

/// Resizes a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDUSize new_size` - The new vector size.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_resize(
    vec: &mut NSTDVec,
    new_size: usize,
) -> NSTDErrorCode {
    if vec.size < new_size {
        if vec.buffer.size < new_size {
            match nstd_collections_vec_reserve(vec, new_size) {
                0 => (),
                errc => return errc,
            }
        }
        let new_bytes = (new_size - vec.size) * vec.buffer.ptr.size;
        std::slice::from_raw_parts_mut(vec.end_unchecked(), new_bytes).fill(0);
    }
    vec.size = new_size;
    0
}

/// Reserves memory for the vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDUSize new_cap` - The new, greater capacity for the vector.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_reserve(
    vec: &mut NSTDVec,
    new_cap: usize,
) -> NSTDErrorCode {
    if vec.buffer.size < new_cap {
        let old_byte_count = vec.total_byte_count();
        let new_byte_count = new_cap * vec.buffer.ptr.size;
        match crate::alloc::nstd_alloc_reallocate(
            &mut vec.buffer.ptr.raw,
            old_byte_count,
            new_byte_count,
        ) {
            0 => {
                vec.buffer.size = new_cap;
                0
            }
            errc => errc,
        }
    } else {
        1
    }
}

/// Shrinks a vector to free any unused memory.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_shrink(vec: &mut NSTDVec) -> NSTDErrorCode {
    if vec.size > 0 {
        let old_byte_count = vec.total_byte_count();
        let new_byte_count = vec.byte_count();
        match crate::alloc::nstd_alloc_reallocate(
            &mut vec.buffer.ptr.raw,
            old_byte_count,
            new_byte_count,
        ) {
            0 => {
                vec.buffer.size = vec.size;
                0
            }
            errc => errc,
        }
    } else {
        1
    }
}

/// Frees a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_collections_vec_free(vec: &mut NSTDVec) -> NSTDErrorCode {
    let bytes = vec.total_byte_count();
    crate::alloc::nstd_alloc_deallocate(&mut vec.buffer.ptr.raw, bytes)
}
