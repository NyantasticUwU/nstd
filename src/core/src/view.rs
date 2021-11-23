use core::{ffi::c_void, ptr};

/// Represents a view into a sequence of data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NSTDView {
    pub size: usize,
    pub element_size: usize,
    pub data: *mut u8,
}
impl NSTDView {
    /// Gets the number of used bytes for this view.
    #[inline]
    pub fn byte_count(&self) -> usize {
        self.size * self.element_size
    }

    /// Returns a pointer to the end of the view.
    #[inline]
    pub unsafe fn end_unchecked(&self) -> *mut u8 {
        self.data.add(self.byte_count())
    }
}
impl<T> AsRef<[T]> for NSTDView {
    #[inline]
    fn as_ref(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.data as *const T, self.size) }
    }
}
impl<T> AsMut<[T]> for NSTDView {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.data as *mut T, self.size) }
    }
}

/// Creates a new view from raw data.
/// Parameters:
///     `const NSTDUSize size` - Number of elements to view.
///     `const NSTDUSize element_size` - Size of each element.
///     `NSTDByte *const data` - Pointer to the raw data.
/// Returns: `NSTDView view` - The new view.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_view_new(
    size: usize,
    element_size: usize,
    data: *mut u8,
) -> NSTDView {
    NSTDView {
        size,
        element_size,
        data,
    }
}

/// Gets a pointer to an element from a view.
/// NOTE: The returned element pointer can quickly become a dangling pointer if the view's memory
/// gets reallocated or deallocated, so it is advised to create a copy of the element after
/// getting it.
/// Parameters:
///     `const NSTDView *const view` - The view.
///     `const NSTDUSize pos` - The position of the element to get.
/// Returns: `void *element` - Pointer to the element.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_view_get(view: &NSTDView, pos: usize) -> *mut c_void {
    match view.size > pos {
        true => view.data.add(pos * view.element_size) as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Gets the first element in the view.
/// NOTE: This function follows the same behaviour rules as `nstd_core_view_get`.
/// Parameters:
///     `const NSTDView *const view` - The view.
/// Returns: `void *element` - Pointer to the first element.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_view_first(view: &NSTDView) -> *mut c_void {
    match view.size > 0 {
        true => view.data as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Gets the last element in the view.
/// NOTE: This function follows the same behaviour rules as `nstd_core_view_get`.
/// Parameters:
///     `const NSTDView *const view` - The view.
/// Returns: `void *element` - Pointer to the last element.
#[inline]
#[no_mangle]
pub unsafe extern "C" fn nstd_core_view_last(view: &NSTDView) -> *mut c_void {
    match view.size > 0 {
        true => view.end_unchecked().sub(view.element_size) as *mut c_void,
        false => ptr::null_mut(),
    }
}

/// Reverses a view's elements.
/// Parameters:
///     `const NSTDView *const view` - The view.
#[no_mangle]
pub unsafe extern "C" fn nstd_core_view_reverse(view: &NSTDView) {
    let mut ptr = view.data;
    let mut data = core::slice::from_raw_parts_mut(ptr, view.byte_count());
    data.reverse();
    for _ in 0..view.size {
        data = core::slice::from_raw_parts_mut(ptr, view.element_size);
        data.reverse();
        ptr = ptr.add(view.element_size);
    }
}
