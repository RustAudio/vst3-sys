use crate::base::tresult;
use crate::vst::AutomationStates::{kReadState, kWriteState};
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

pub enum AutomationStates {
    kNoAutomation = 0,
    kReadState = 1,
    kWriteState = 2,
    kReadWriteState = kReadState as isize | kWriteState as isize,
}

#[com_interface("B4E8287F-1BB3-46AA-83A4-666768937BAB")]
pub trait IAutomationState: IUnknown {
    unsafe fn set_automation_state(&self, state: i32) -> tresult;
}
