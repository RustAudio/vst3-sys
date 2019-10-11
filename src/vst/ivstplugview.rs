use crate::base::tresult;
use com::com_interface;
use com::interfaces::iunknown::IUnknown;

#[com_interface(0F618302-215D-4587-A512-073C77B9D383)]
pub trait IParemterFinder: IUnknown {
    unsafe fn find_parameter(&self, xpos: i32, ypos: i32, result_tag: *mut u32) -> tresult;
}
