use crate::base::{CStringA, TPtrInt};
use vst3_com::{c_void, com_interface, interfaces::iunknown::IUnknown};

#[com_interface("79655E36-77EE-4267-A573-FEF74912C27C")]
pub trait IReaperHostApplication: IUnknown {
    unsafe fn get_reaper_api(&self, funcname: CStringA) -> *mut c_void;
    unsafe fn get_reaper_parent(&self, w: u32) -> *mut c_void;
    unsafe fn reaper_extended(
        &self,
        call: u32,
        parm1: *mut c_void,
        parm2: *mut c_void,
        parm3: *mut c_void,
    ) -> *mut c_void;
}

#[com_interface("049bf9e7-bc74-ead0-c410-1e867f725981")]
pub trait IReaperUIEmbedInterface: IUnknown {
    unsafe fn embed_message(&self, message: i32, parm2: TPtrInt, parm3: TPtrInt) -> TPtrInt;
}
