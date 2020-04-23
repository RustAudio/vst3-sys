//! Raw Bindings to the VST3 API.
//!
//! This crate is a work in progress. See the examples for usage, and the [official API docs](https://steinbergmedia.github.io/vst3_doc/)
//! for reference.
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]

pub mod base;
pub mod gui;
pub mod vst;
pub use vst3_com::co_class as VST3;
pub use vst3_com::REFIID;
pub use vst3_com::*;
