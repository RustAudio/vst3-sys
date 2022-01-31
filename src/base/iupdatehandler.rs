use crate::base::tresult;
use crate::utils::SharedVstPtr;

use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

#[com_interface("F5246D56-8654-4d60-B026-AFB57B697B37")]
pub trait IUpdateHandler: IUnknown {
    unsafe fn add_dependent(
        &self,
        object: SharedVstPtr<dyn IUnknown>,
        dependent: SharedVstPtr<dyn IDependent>,
    ) -> tresult;
    unsafe fn remove_dependent(
        &self,
        object: SharedVstPtr<dyn IUnknown>,
        dependent: SharedVstPtr<dyn IDependent>,
    ) -> tresult;
    unsafe fn trigger_updates(&self, object: SharedVstPtr<dyn IUnknown>, message: i32) -> tresult;
    unsafe fn defer_updates(&self, object: SharedVstPtr<dyn IUnknown>, message: i32) -> tresult;
}

#[com_interface("F52B7AAE-DE72-416d-8AF1-8ACE9DD7BD5E")]
pub trait IDependent: IUnknown {
    unsafe fn update(&self, changed_unknown: SharedVstPtr<dyn IUnknown>, message: i32);
}

pub const kWillChange: i32 = 0;
pub const kChanged: i32 = 1;
pub const kDestroyed: i32 = 2;
pub const kWillDestroy: i32 = 3;
pub const kStdChangeMessageLast: i32 = kWillDestroy;
