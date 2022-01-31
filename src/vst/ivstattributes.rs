use crate::base::{char8, tchar, tresult};
use crate::utils::StaticVstPtr;
use vst3_com::interfaces::iunknown::IUnknown;
use vst3_com::{c_void, com_interface};

pub type AttrID = *const char8;

#[com_interface("1E5F0AEB-CC7F-4533-A254-401138AD5EE4")]
pub trait IAttributeList: IUnknown {
    unsafe fn set_int(&self, id: AttrID, value: i64) -> tresult;
    unsafe fn get_int(&self, id: AttrID, value: *mut i64) -> tresult;
    unsafe fn set_float(&self, id: AttrID, value: f64) -> tresult;
    unsafe fn get_float(&self, id: AttrID, value: *mut f64) -> tresult;
    unsafe fn set_string(&self, id: AttrID, value: *const tchar, size: u32) -> tresult;
    unsafe fn get_string(&self, id: AttrID, value: *mut tchar, size: u32) -> tresult;
    unsafe fn set_binary(&self, id: AttrID, ptr: *const c_void, size: u32) -> tresult;
    unsafe fn get_binary(&self, id: AttrID, ptr: *const *mut c_void, size: *mut u32) -> tresult;
}

#[com_interface("D6CE2FFC-EFAF-4B8C-9E74-F1BB12DA44B4")]
pub trait IStreamAttributes: IUnknown {
    unsafe fn get_filename(&self, name: *const tchar) -> tresult;
    unsafe fn get_attributes(&self) -> StaticVstPtr<dyn IAttributeList>;
}
