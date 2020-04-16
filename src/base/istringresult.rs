use crate::base::{char16, char8};
use com::interfaces::iunknown::IUnknown;
use com::{c_void, com_interface};

#[com_interface("550798BC-8720-49DB-8492-0A153B50B7A8")]
pub trait IStringResult: IUnknown {
    unsafe fn set_text(&self, text: *const char8);
}

#[com_interface("F99DB7A3-0FC1-4821-800B-0CF98E348EDF")]
pub trait IString: IUnknown {
    unsafe fn set_text8(&self, text: *const char8);
    unsafe fn set_text16(&self, text: *const char16);
    unsafe fn get_text8(&self) -> *const char8;
    unsafe fn get_text16(&self) -> *const char16;
    unsafe fn take(&self, _s: *mut c_void, _is_wide: bool) {}
    unsafe fn is_wide_string(&self) -> bool;
}
