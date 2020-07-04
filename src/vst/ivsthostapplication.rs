use crate::base::tresult;
use crate::vst::String128;
use vst3_com::interfaces::iunknown::IUnknown;
use vst3_com::{c_void, com_interface, IID};

#[com_interface("58E595CC-DB2D-4969-8B6A-AF8C36A664E5")]
pub trait IHostApplication: IUnknown {
    unsafe fn get_name(&self, name: *mut u16) -> tresult;
    unsafe fn create_instance(&self, cid: IID, _iid: IID, obj: *mut *mut c_void) -> tresult;
}
