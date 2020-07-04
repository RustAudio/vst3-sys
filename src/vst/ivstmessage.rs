use crate::base::{tresult, FIDString};
use crate::vst::IAttributeList;
use vst3_com::interfaces::iunknown::IUnknown;
use vst3_com::{c_void, com_interface};

#[com_interface("936F033B-C6C0-47DB-BB08-82F813C1E613")]
pub trait IMessage: IUnknown {
    unsafe fn get_message_id(&self) -> FIDString;
    unsafe fn set_message_id(&self, id: FIDString);
    unsafe fn get_attributes(&self) -> *mut c_void;
}

#[com_interface("70A4156F-6E6E-4026-9891-48BFAA60D8D1")]
pub trait IConnectionPoint: IUnknown {
    unsafe fn connect(&self, other: *mut c_void) -> tresult;
    unsafe fn disconnect(&self, other: *mut c_void) -> tresult;
    unsafe fn notify(&self, message: *mut c_void) -> tresult;
}
