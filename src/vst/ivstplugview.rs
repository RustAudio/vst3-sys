use crate::base::tresult;
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

#[com_interface("0F618302-215D-4587-A512-073C77B9D383")]
pub trait IParameterFinder: IUnknown {
    unsafe fn find_parameter(&self, xpos: i32, ypos: i32, result_tag: *mut u32) -> tresult;
}
