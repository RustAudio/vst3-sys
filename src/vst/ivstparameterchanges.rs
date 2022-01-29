use crate::base::tresult;
use crate::utils::SharedVstPtr;
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

#[com_interface("01263A18-ED07-4F6F-98C9-D3564686F9BA")]
pub trait IParamValueQueue: IUnknown {
    unsafe fn get_parameter_id(&self) -> u32;
    unsafe fn get_point_count(&self) -> i32;
    unsafe fn get_point(&self, index: i32, sample_offset: *mut i32, value: *mut f64) -> tresult;
    unsafe fn add_point(&self, sample_offset: i32, value: f64, index: *mut i32) -> tresult;
}

#[com_interface("A4779663-0BB6-4A56-B443-84A8466FEB9D")]
pub trait IParameterChanges: IUnknown {
    unsafe fn get_parameter_count(&self) -> i32;
    // Returns a pointer to IParamValueQueue
    unsafe fn get_parameter_data(&self, index: i32) -> SharedVstPtr<dyn IParamValueQueue>;
    // Returns a pointer to IParamValueQueue
    unsafe fn add_parameter_data(
        &self,
        id: *const u32,
        index: *mut i32,
    ) -> SharedVstPtr<dyn IParameterChanges>;
}
