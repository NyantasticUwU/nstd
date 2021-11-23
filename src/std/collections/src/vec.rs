use std::{
    mem::ManuallyDrop,
    os::raw::{c_int, c_void},
    ptr,
};

/// Represents an array of dynamic length.
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NSTDVec {
    pub size: usize,
    pub capacity: usize,
    pub element_size: usize,
    pub data: *mut u8,
}
impl NSTDVec {
    /// Gets the total number of bytes allocated for this vec.
    #[inline]
    pub fn total_byte_count(&self) -> usize {
        self.capacity * self.element_size
    }

    /// Gets the number of used bytes for this vec.
    #[inline]
    pub fn byte_count(&self) -> usize {
        self.size * self.element_size
    }

    /// Returns a pointer to the end of the vector.
    #[inline]
    pub unsafe fn end_unchecked(&self) -> *mut u8 {
        self.data.add(self.byte_count())
    }

    /// Drops elements aquired from a [`Vec`].
    pub unsafe fn drop_from_vec<T>(&mut self) -> c_int {
        let data_ptr = self.data as *mut ManuallyDrop<T>;
        let data_slice = std::slice::from_raw_parts_mut(data_ptr, self.size);
        for element in data_slice {
            ManuallyDrop::<T>::drop(element);
        }
        nstd_std_collections_vec_free(self)
    }
}
impl Default for NSTDVec {
    #[inline]
    fn default() -> Self {
        Self {
            size: 0,
            capacity: 0,
            element_size: 0,
            data: ptr::null_mut(),
        }
    }
}
impl Clone for NSTDVec {
    fn clone(&self) -> Self {
        unsafe {
            let mut new_vec =
                nstd_std_collections_vec_new_with_capacity(self.element_size, self.capacity);
            if !new_vec.data.is_null() {
                let byte_count = self.byte_count();
                let data = std::slice::from_raw_parts(self.data, byte_count);
                let new_data = std::slice::from_raw_parts_mut(new_vec.data, byte_count);
                new_data.copy_from_slice(data);
                new_vec.size = self.size;
            }
            new_vec
        }
    }
}
impl<T> From<Vec<T>> for NSTDVec {
    /// Creates an `NSTDVec` from a [`Vec`]. The only way to free memory allocated by this
    /// function is to call `drop_from_vec`.
    fn from(vec: Vec<T>) -> Self {
        unsafe {
            let element_size = std::mem::size_of::<ManuallyDrop<T>>();
            let mut nstd_vec = nstd_std_collections_vec_new_with_capacity(element_size, vec.len());
            if !nstd_vec.data.is_null() {
                for element in vec {
                    let element = ManuallyDrop::new(element);
                    let element = &element as *const ManuallyDrop<T> as *const c_void;
                    nstd_std_collections_vec_push(&mut nstd_vec, element);
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
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_new(element_size: usize) -> NSTDVec {
    const INITIAL_CAPACITY: usize = 1;
    NSTDVec {
        size: 0,
        capacity: INITIAL_CAPACITY,
        element_size,
        data: nstd_alloc::nstd_std_alloc_allocate(INITIAL_CAPACITY * element_size),
    }
}

/// Creates a new vector with the specified capacity.
/// Parameters:
///     `const NSTDUSize element_size` - The size of each element in the vector.
///     `const NSTDUSize capacity` - The capacity to give the vector, must be greater than 0.
/// Returns: `NSTDVec vec` - The new vector.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_new_with_capacity(
    element_size: usize,
    capacity: usize,
) -> NSTDVec {
    NSTDVec {
        size: 0,
        capacity,
        element_size,
        data: nstd_alloc::nstd_std_alloc_allocate(capacity * element_size),
    }
}

/// Gets a pointer to an element from a vector.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the vector's memory
/// gets reallocated or deallocated, so it is advised to create a copy of the element after
/// getting it.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
///     `const NSTDUSize pos` - The position of the element to get.
/// Returns: `void *element` - Pointer to the element.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_get(vec: &NSTDVec, pos: usize) -> *mut c_void {
    match vec.size > pos {
        true => vec.data.add(pos * vec.element_size) as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Gets the first element in the vector.
/// NOTE: This function follows the same behaviour rules as `nstd_std_collections_vec_get`.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
/// Returns: `void *element` - Pointer to the first element.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_first(vec: &NSTDVec) -> *mut c_void {
    match vec.size > 0 {
        true => vec.data as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Gets the last element in the vector.
/// NOTE: This function follows the same behaviour rules as `nstd_std_collections_vec_get`.
/// Parameters:
///     `const NSTDVec *const vec` - The vector.
/// Returns: `void *element` - Pointer to the last element.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_last(vec: &NSTDVec) -> *mut c_void {
    match vec.size > 0 {
        true => vec.end_unchecked().sub(vec.element_size) as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Pushes a new element onto the end of a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const void *const element` - Pointer to the new element.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_push(
    vec: &mut NSTDVec,
    element: *const c_void,
) -> c_int {
    // Checking if the vector has reached it's capacity.
    if vec.size == vec.capacity {
        let new_cap = (vec.capacity as f32 * 1.5).ceil() as usize;
        match nstd_std_collections_vec_reserve(vec, new_cap) {
            0 => (),
            errc => return errc,
        }
        vec.capacity = new_cap;
    }
    // Adding new element.
    let element = std::slice::from_raw_parts(element as *const u8, vec.element_size);
    let data = std::slice::from_raw_parts_mut(vec.end_unchecked(), vec.element_size);
    data.copy_from_slice(element);
    vec.size += 1;
    0
}

/// Pops a value off of the back of a vector and returns a pointer to it.
/// NOTE: This function follows the same behaviour rules as `nstd_std_collections_vec_get`.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
/// Returns: `void *element` - The element that was removed.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_pop(vec: &mut NSTDVec) -> *mut c_void {
    match vec.size > 0 {
        true => {
            vec.size -= 1;
            vec.end_unchecked() as *mut c_void
        }
        false => ptr::null_mut(),
    }
}

/// Inserts an element at `index` for a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const void *const element` - Pointer to the new element.
///     `const NSTDUSize index` - The index to insert an element.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_insert(
    vec: &mut NSTDVec,
    element: *const c_void,
    index: usize,
) -> c_int {
    // A value is being inserted.
    if vec.size > index {
        // Checking if the vector has reached it's capacity.
        if vec.size == vec.capacity {
            match nstd_std_collections_vec_reserve(vec, vec.capacity + 1) {
                0 => (),
                errc => return errc,
            }
        }
        // Moving data up by one element.
        let index_pointer = nstd_std_collections_vec_get(vec, index) as *mut u8;
        let next_index_pointer = index_pointer.add(vec.element_size);
        let copy_size = (vec.size - index) * vec.element_size;
        ptr::copy(index_pointer, next_index_pointer, copy_size);
        // Inserting data.
        let element = std::slice::from_raw_parts(element as *const u8, vec.element_size);
        let data = std::slice::from_raw_parts_mut(index_pointer, vec.element_size);
        data.copy_from_slice(element);
        vec.size += 1;
        0
    }
    // A value is being pushed.
    else if vec.size == index {
        nstd_std_collections_vec_push(vec, element)
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
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_remove(vec: &mut NSTDVec, index: usize) -> c_int {
    match vec.size > index {
        true => {
            // Moving data down by one element.
            let index_pointer = nstd_std_collections_vec_get(vec, index) as *mut u8;
            let next_index_pointer = index_pointer.add(vec.element_size);
            let copy_size = (vec.size - index - 1) * vec.element_size;
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
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_clear(vec: &mut NSTDVec) {
    vec.size = 0;
}

/// Resizes a vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDUSize new_size` - The new vector size.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_resize(
    vec: &mut NSTDVec,
    new_size: usize,
) -> c_int {
    if vec.size < new_size {
        if vec.capacity < new_size {
            match nstd_std_collections_vec_reserve(vec, new_size) {
                0 => (),
                errc => return errc,
            }
        }
        let new_bytes = (new_size - vec.size) * vec.element_size;
        std::slice::from_raw_parts_mut(vec.end_unchecked(), new_bytes).fill(0);
    }
    vec.size = new_size;
    0
}

/// Reserves memory for the vector.
/// Parameters:
///     `NSTDVec *const vec` - The vector.
///     `const NSTDUSize new_cap` - The new, greater capacity for the vector.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_reserve(
    vec: &mut NSTDVec,
    new_cap: usize,
) -> c_int {
    if vec.capacity < new_cap {
        let old_byte_count = vec.total_byte_count();
        let new_byte_count = new_cap * vec.element_size;
        match nstd_alloc::nstd_std_alloc_reallocate(&mut vec.data, old_byte_count, new_byte_count) {
            0 => {
                vec.capacity = new_cap;
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
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_shrink(vec: &mut NSTDVec) -> c_int {
    if vec.size > 0 {
        let old_byte_count = vec.total_byte_count();
        let new_byte_count = vec.byte_count();
        match nstd_alloc::nstd_std_alloc_reallocate(&mut vec.data, old_byte_count, new_byte_count) {
            0 => {
                vec.capacity = vec.size;
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
/// Returns: `int errc` - Nonzero on error.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_std_collections_vec_free(vec: &mut NSTDVec) -> c_int {
    nstd_alloc::nstd_std_alloc_deallocate(&mut vec.data, vec.total_byte_count())
}
