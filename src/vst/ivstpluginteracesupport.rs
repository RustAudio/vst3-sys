use crate::base::tresult;
use vst3_com::interfaces::iunknown::IUnknown;
use vst3_com::{com_interface, REFIID};

#[com_interface("4FB58B9E-9EAA-4E0F-AB36-1C1CCCB56FEA")]
pub trait IPlugInterfaceSupport: IUnknown {
    unsafe fn is_pluginterface_supported(&self, _iid: REFIID) -> tresult;
}
