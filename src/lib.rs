#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#[cfg(feature = "nstd_alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_alloc")))]
pub mod alloc;
#[cfg(feature = "nstd_audio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_audio")))]
pub mod audio;
#[cfg(feature = "nstd_collections")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_collections")))]
pub mod collections;
#[cfg(feature = "nstd_core")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_core")))]
pub mod core;
#[cfg(feature = "nstd_env")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_env")))]
pub mod env;
#[cfg(feature = "nstd_events")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_events")))]
pub mod events;
#[cfg(feature = "nstd_fs")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_fs")))]
pub mod fs;
#[cfg(feature = "nstd_gl")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_gl")))]
pub mod gl;
#[cfg(feature = "nstd_gui")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_gui")))]
pub mod gui;
#[cfg(feature = "nstd_image")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_image")))]
pub mod image;
#[cfg(feature = "nstd_input")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_input")))]
pub mod input;
#[cfg(feature = "nstd_io")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_io")))]
pub mod io;
#[cfg(feature = "nstd_math")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_math")))]
pub mod math;
#[cfg(feature = "nstd_net")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_net")))]
pub mod net;
#[cfg(feature = "nstd_os")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_os")))]
pub mod os;
#[cfg(feature = "nstd_proc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_proc")))]
pub mod proc;
#[cfg(feature = "nstd_rand")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_rand")))]
pub mod rand;
#[cfg(feature = "nstd_string")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_string")))]
pub mod string;
#[cfg(feature = "nstd_thread")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_thread")))]
pub mod thread;
#[cfg(feature = "nstd_time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_time")))]
pub mod time;
#[cfg(feature = "nstd_vec")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_vec")))]
pub mod vec;
