#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_imports)]
pub mod base;
pub mod gui;
pub mod vst;
pub use vst3_com::co_class as VST3;
pub use vst3_com::REFIID;
use vst3_com::*;
