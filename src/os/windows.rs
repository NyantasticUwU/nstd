//! OS support for Windows.
#[cfg(feature = "nstd_os_alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_os_alloc")))]
pub mod alloc;
#[cfg(feature = "nstd_os_def")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_os_def")))]
pub mod def;
#[cfg(feature = "nstd_os_io")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_os_io")))]
pub mod io;
