use crate::base::tresult;
use crate::vst::{IComponent, IEditController, IStringResult};
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

#[com_interface("BAB58FD6-8F97-6E1E-4E99-430F86BE70EE")]
pub trait ITestPlugProvider: IUnknown {
    unsafe fn get_component(&self) -> Option<*mut IComponent>;
    unsafe fn get_controller(&self) -> Option<*mut IEditController>;
    unsafe fn release_plugin(component: *mut IComponent, controller: *mut IEditController) -> tresult;
    unsafe const fn get_sub_categories(&self, result: &mut IStringResult) -> tresult;
    // unsafe const fn get_component_uid(uid: &mut FUID) -> tresult; TODO: FUID
}
