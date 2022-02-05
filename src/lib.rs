#![cfg_attr(not(any(test, feature = "std")), no_std)]
#[cfg(feature = "nstd_alloc")]
pub mod alloc;
#[cfg(feature = "nstd_audio")]
pub mod audio;
#[cfg(feature = "nstd_collections")]
pub mod collections;
#[cfg(feature = "nstd_core")]
pub mod core;
#[cfg(feature = "nstd_env")]
pub mod env;
#[cfg(feature = "nstd_events")]
pub mod events;
#[cfg(feature = "nstd_fs")]
pub mod fs;
#[cfg(feature = "nstd_gl")]
pub mod gl;
#[cfg(feature = "nstd_gui")]
pub mod gui;
#[cfg(feature = "nstd_image")]
pub mod image;
#[cfg(feature = "nstd_input")]
pub mod input;
#[cfg(feature = "nstd_io")]
pub mod io;
#[cfg(feature = "nstd_math")]
pub mod math;
#[cfg(feature = "nstd_net")]
pub mod net;
#[cfg(feature = "nstd_proc")]
pub mod proc;
#[cfg(feature = "nstd_str")]
pub mod str;
#[cfg(feature = "nstd_sys")]
pub mod sys;
#[cfg(feature = "nstd_thread")]
pub mod thread;
#[cfg(feature = "nstd_time")]
pub mod time;
