use crate::base::{tresult, FIDString, IPluginBase};
use crate::vst::{ParamID, ParamValue, String128, TChar};
use vst3_com::interfaces::IUnknown;
use vst3_com::{c_void, com_interface};

pub enum ParameterFlags {
    kNoFlags = 0,
    kCanAutomate = 1,
    kIsReadOnly = 1 << 1,
    kIsWrapAround = 1 << 2,
    kIsList = 1 << 3,
    kIsProgramChange = 1 << 15,
    kIsBypass = 1 << 16,
}

// todo: update types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ParameterInfo {
    pub id: u32, // ParamID
    pub title: String128,
    pub short_title: String128,
    pub units: String128,
    pub step_count: i32,
    pub default_normalized_value: f64, // ParamValue
    pub unit_id: i32,                  // UnitID
    pub flags: i32,
}

/// Host callback interface for an edit controller.
#[com_interface("93A0BEA3-0BD0-45DB-8E89-0B0CC1E46AC6")]
pub trait IComponentHandler: IUnknown {
    unsafe fn begin_edit(&self, id: ParamID) -> tresult;
    unsafe fn perform_edit(&self, id: ParamID, value_normalized: ParamValue) -> tresult;
    unsafe fn end_edit(&self, id: ParamID) -> tresult;
    unsafe fn restart_component(&self, flags: i32) -> tresult;
}

// todo: update types
/// Edit controller component interface.
#[com_interface("DCD7BBE3-7742-448D-A874-AACC979C759E")]
pub trait IEditController: IPluginBase {
    unsafe fn set_component_state(&mut self, state: *mut c_void) -> tresult;
    unsafe fn set_state(&mut self, state: *mut c_void) -> tresult;
    unsafe fn get_state(&mut self, state: *mut c_void) -> tresult;
    unsafe fn get_parameter_count(&self) -> i32;
    unsafe fn get_parameter_info(&self, param_index: i32, info: *mut ParameterInfo) -> tresult;
    unsafe fn get_param_string_by_value(
        &self,
        id: u32,
        value_normalized: f64,
        string: *mut TChar,
    ) -> tresult;
    unsafe fn get_param_value_by_string(
        &self,
        id: u32,
        string: *mut TChar,
        value_normalized: *mut f64,
    ) -> tresult;
    unsafe fn normalized_param_to_plain(&self, id: u32, value_normalized: f64) -> f64;
    unsafe fn plain_param_to_normalized(&self, id: u32, plain_value: f64) -> f64;
    unsafe fn get_param_normalized(&self, id: u32) -> f64;
    unsafe fn set_param_normalized(&mut self, id: u32, value: f64) -> tresult;
    unsafe fn set_component_handler(&mut self, handler: *mut c_void) -> tresult;
    unsafe fn create_view(&self, name: FIDString) -> *mut c_void;
}
