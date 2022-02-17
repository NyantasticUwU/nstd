/// Represents a touch screen's state.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDTouchState {
    /// The press was moved.
    NSTD_TOUCH_STATE_MOVED,
    /// A press has started.
    NSTD_TOUCH_STATE_STARTED,
    /// A press has ended
    NSTD_TOUCH_STATE_ENDED,
    /// A press was canceled.
    NSTD_TOUCH_STATE_CANCELLED,
}
impl Default for NSTDTouchState {
    #[inline]
    fn default() -> Self {
        NSTDTouchState::NSTD_TOUCH_STATE_MOVED
    }
}
