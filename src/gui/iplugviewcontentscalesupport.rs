use crate::base::tresult;
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

#[com_interface("65ED9690-8AC4-4525-8AAD-EF7A72EA703F")]
pub trait IPlugViewContentScaleSupport: IUnknown {
    unsafe fn set_scale_factor(&self, factor: f32) -> tresult;
}
