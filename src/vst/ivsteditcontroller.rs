use crate::base::{tchar, tresult, FIDString, IBStream, IPluginBase, TBool};
use crate::utils::SharedVstPtr;
use crate::vst::{
    BusDirection, CString, CtrlNumber, KnobMode, MediaType, ParamID, ParamValue, String128, TChar,
};
use vst3_com::interfaces::IUnknown;
use vst3_com::{c_void, com_interface};

pub const kVstComponentControllerClass: CString =
    b"Component Controller Class\0".as_ptr() as *const _;

pub enum RestartFlags {
    kReloadComponent = 1,
    kIoChanged = 1 << 1,
    kParamValuesChanged = 1 << 2,
    kLatencyChanged = 1 << 3,
    kParamTitlesChanged = 1 << 4,
    kMidiCCAssignmentChanged = 1 << 5,
    kNoteExpressionChanged = 1 << 6,
    kIoTitlesChanged = 1 << 7,
    kPrefetchableSupportChanged = 1 << 8,
    kRoutingInfoChanged = 1 << 9,
}

pub enum ParameterFlags {
    kNoFlags = 0,
    kCanAutomate = 1,
    kIsReadOnly = 1 << 1,
    kIsWrapAround = 1 << 2,
    kIsList = 1 << 3,
    kIsProgramChange = 1 << 15,
    kIsBypass = 1 << 16,
}

pub enum KnobModes {
    kCircularMode = 0,
    kRelativCircularMode = 1,
    kLinearMode = 2,
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
    pub default_normalized_value: f64,
    pub unit_id: i32,
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
    unsafe fn set_component_state(&self, state: SharedVstPtr<dyn IBStream>) -> tresult;
    unsafe fn set_state(&self, state: SharedVstPtr<dyn IBStream>) -> tresult;
    unsafe fn get_state(&self, state: SharedVstPtr<dyn IBStream>) -> tresult;
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
        string: *const TChar,
        value_normalized: *mut f64,
    ) -> tresult;
    unsafe fn normalized_param_to_plain(&self, id: u32, value_normalized: f64) -> f64;
    unsafe fn plain_param_to_normalized(&self, id: u32, plain_value: f64) -> f64;
    unsafe fn get_param_normalized(&self, id: u32) -> f64;
    unsafe fn set_param_normalized(&self, id: u32, value: f64) -> tresult;
    unsafe fn set_component_handler(&self, handler: SharedVstPtr<dyn IComponentHandler>) -> tresult;
    unsafe fn create_view(&self, name: FIDString) -> *mut c_void;
}

#[com_interface("7F4EFE59-F320-4967-AC27-A3AEAFB63038")]
pub trait IEditController2: IUnknown {
    unsafe fn set_knob_mode(&self, mode: KnobMode) -> tresult;
    unsafe fn open_help(&self, only_check: TBool) -> tresult;
    unsafe fn oepn_about_box(&self, only_check: TBool) -> tresult;
}

#[com_interface("DF0FF9F7-49B7-4669-B63A-B7327ADBF5E5")]
pub trait IMidiMapping: IUnknown {
    unsafe fn get_midi_controller_assignment(
        &self,
        bus_index: i32,
        channel: i16,
        midi_cc_number: CtrlNumber,
        param_id: *mut ParamID,
    ) -> tresult;
}

#[com_interface("067D02C1-5B4E-274D-A92D-90FD6EAF7240")]
pub trait IComponentHandlerBusActivation: IUnknown {
    unsafe fn request_bus_activation(
        &self,
        media_type: MediaType,
        direction: BusDirection,
        state: TBool,
    ) -> tresult;
}

#[com_interface("C1271208-7059-4098-B9DD-34B36BB0195E")]
pub trait IEditControllerHostEditing: IUnknown {
    unsafe fn begin_edit_from_host(&self, id: ParamID) -> tresult;
    unsafe fn end_edit_from_host(&self, id: ParamID) -> tresult;
}

#[allow(non_snake_case)]
pub mod ProgressType {
    pub const AsyncStateRestoration: u32 = 0;
    pub const UIBackgroundTask: u32 = 1;
}

#[com_interface("00C9DC5B-9D90-4254-91A3-88C8B4E91B69")]
pub trait IProgress: IUnknown {
    unsafe fn start(
        &self,
        progress_type: u32,
        optional_description: *const tchar,
        out_id: *mut u64,
    ) -> i32; // fucking _why_
    unsafe fn update(&self, id: u64, value: f64);
    unsafe fn finish(&self, id: u64);
}
