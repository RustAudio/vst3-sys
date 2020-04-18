use crate::base::tresult;
use bitflags::bitflags;
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

bitflags! {
    pub struct AutomationStates: i32 {
        const kNoAutomation = 0;
        const kReadState = 1;
        const kWriteState = 2;
        const kReadWriteState = AutomationStates::kReadState.bits | AutomationStates::kWriteState.bits;
    }
}

#[com_interface("B4E8287F-1BB3-46AA-83A4-666768937BAB")]
pub trait IAutomationState: IUnknown {
    unsafe fn set_automation_state(&self, state: i32) -> tresult;
}
