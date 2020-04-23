use super::IUnknown;
use crate::base::{tresult, IString};
use vst3_com::com_interface;

#[com_interface("12BCD07B-7C69-4336-B7DA-77C3444A0CD0")]
pub trait IErrorContext: IUnknown {
    unsafe fn disable_error_ui(&self, state: bool);
    unsafe fn error_message_shown(&self) -> tresult;
    unsafe fn get_error_message(&self, message: *mut dyn IString) -> tresult;
}
