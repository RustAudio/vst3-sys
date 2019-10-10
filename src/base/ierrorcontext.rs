use super::IUnknown;
use crate::base::{tresult, IString};
use com::{c_void, com_interface};

#[com_interface(12BCD07B-7C69-4336-B7DA-77C3444A0CD0)]
pub trait IErrorContext: IUnknown {
    fn disable_error_ui(&self, state: bool);
    fn error_message_shown(&self) -> tresult;
    fn get_error_message(&self, message: *mut dyn IString) -> tresult;
}
