use crate::base::tresult;
use com::interfaces::iunknown::IUnknown;
use com::{com_interface, REFIID};

//todo: fix parse error
//#[com_interface(4FB58B9E-9EAA-4E0F-AB36-1C1CCCB56FEA)]
pub trait IPlugInterfaceSupport: IUnknown {
    unsafe fn is_pluginterface_supported(&self, _iid: REFIID) -> tresult;
}
