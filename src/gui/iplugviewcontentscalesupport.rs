use crate::base::tresult;
use com::com_interface;
use com::interfaces::iunknown::IUnknown;

//todo: fix parse error
//#[com_interface(65ED9690-8AC4-4525-8AAD-EF7A72EA703F)]
pub trait IPlugViewContentScaleSupport: IUnknown {
    fn set_scale_factor(&self, factor: f32) -> tresult;
}
