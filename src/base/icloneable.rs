use vst3_com::{com_interface, interfaces::iunknown::IUnknown};

#[com_interface("D45406B9-3A2D-4443-9DAD-9BA985A1454B")]
pub trait ICloneable: IUnknown {
    unsafe fn clone(&self) -> *mut dyn IUnknown;
}
