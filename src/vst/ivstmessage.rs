use crate::base::{tresult, FIDString};
use crate::utils::SharedVstPtr;
use crate::vst::IAttributeList;
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

#[com_interface("936F033B-C6C0-47DB-BB08-82F813C1E613")]
pub trait IMessage: IUnknown {
    unsafe fn get_message_id(&self) -> FIDString;
    unsafe fn set_message_id(&self, id: FIDString);
    unsafe fn get_attributes(&self) -> SharedVstPtr<dyn IAttributeList>;
}

#[com_interface("70A4156F-6E6E-4026-9891-48BFAA60D8D1")]
pub trait IConnectionPoint: IUnknown {
    unsafe fn connect(&self, other: SharedVstPtr<dyn IConnectionPoint>) -> tresult;
    unsafe fn disconnect(&self, other: SharedVstPtr<dyn IConnectionPoint>) -> tresult;
    unsafe fn notify(&self, message: SharedVstPtr<dyn IMessage>) -> tresult;
}
