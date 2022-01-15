pub mod vec;
#[cfg(feature = "deps")]
pub mod deps {
    pub use nstd_alloc;
}
