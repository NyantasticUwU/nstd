/// Represents a touch screen's state.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDTouchState {
    NSTD_TOUCH_STATE_MOVED,
    NSTD_TOUCH_STATE_STARTED,
    NSTD_TOUCH_STATE_ENDED,
    NSTD_TOUCH_STATE_CANCELLED,
}
impl Default for NSTDTouchState {
    #[inline]
    fn default() -> Self {
        NSTDTouchState::NSTD_TOUCH_STATE_MOVED
    }
}
