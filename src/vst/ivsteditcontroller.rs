use crate::base::{tresult, FIDString, IPluginBase, TBool};
use crate::vst::{
    BusDirection, CString, CtrlNumber, MediaType, ParamID, ParamValue, String128, TChar,
};
use vst3_com::interfaces::IUnknown;
use vst3_com::{c_void, com_interface};

pub const kVstComponentControllerClass: CString =
    b"Component Controller Class\0".as_ptr() as *const _;

pub enum ParameterFlags {
    kNoFlags = 0,
    kCanAutomate = 1,
    kIsReadOnly = 1 << 1,
    kIsWrapAround = 1 << 2,
    kIsList = 1 << 3,
    kIsProgramChange = 1 << 15,
    kIsBypass = 1 << 16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ParameterInfo {
    pub id: i32,
    pub title: String128,
    pub short_title: String128,
    pub units: String128,
    pub step_count: i32,
    pub default_normalized_value: f64,
    pub unit_id: i32,
    pub flags: i32,
}

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

#[com_interface("93A0BEA3-0BD0-45DB-8E89-0B0CC1E46AC6")]
pub trait IComponentHandler: IUnknown {
    unsafe fn begin_edit(&self, id: ParamID) -> tresult;
    unsafe fn perform_edit(&self, id: ParamID, value_normalized: ParamValue) -> tresult;
    unsafe fn end_edit(&self, id: ParamID) -> tresult;
    unsafe fn restart_component(&self, flags: i32) -> tresult;
}

#[com_interface("F040B4B3-A360-45EC-ABCD-C045B4D5A2CC")]
pub trait IComponentHandler2: IUnknown {
    unsafe fn set_dirty(&self, state: TBool) -> tresult;
    unsafe fn request_open_editor(&self, name: *const u8) -> tresult;
    unsafe fn start_group_edit(&self) -> tresult;
    unsafe fn finish_group_edit(&self) -> tresult;
}

#[com_interface("067D02C1-5B4E-274D-A92D-90FD6EAF7240")]
pub trait IComponentHandlerBusActivation: IUnknown {
    unsafe fn request_bus_activation(
        &self,
        type_: MediaType,
        dir: BusDirection,
        index: i32,
        state: TBool,
    ) -> tresult;
}

#[com_interface("DCD7BBE3-7742-448D-A874-AACC979C759E")]
pub trait IEditController: IPluginBase {
    unsafe fn set_component_state(&self, state: *mut c_void) -> tresult;
    unsafe fn set_state(&self, state: *mut c_void) -> tresult;
    unsafe fn get_state(&self, state: *mut c_void) -> tresult;
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
    unsafe fn set_component_handler(&self, handler: *mut c_void) -> tresult;
    unsafe fn create_view(&self, name: FIDString) -> *mut c_void;
}

pub enum KnobModes {
    kCircularMode = 0,
    kRelativeCircularMode = 1,
    kLinearMode = 2,
}

type KnobMode = i32;

#[com_interface("7F4EFE59-F320-4967-AC27-A3AEAFB63038")]
pub trait IEditController2: IUnknown {
    unsafe fn set_knob_mode(&self, mode: KnobMode) -> tresult;
    unsafe fn open_help(&self, only_check: TBool) -> tresult;
    unsafe fn open_about_box(&self, only_check: TBool) -> tresult;
}

#[com_interface("DF0FF9F7-49B7-4669-B63A-B7327ADBF5E5")]
pub trait IMidiMapping: IUnknown {
    unsafe fn get_midi_controller_assignment(
        &self,
        bus_index: i32,
        channel: i16,
        midi_controller_number: CtrlNumber,
        id: *mut u32,
    ) -> tresult;
}

#[com_interface("C1271208-7059-4098-B9DD-34B36BB0195E")]
pub trait IEditControllerHostEditing: IUnknown {
    unsafe fn begin_edit_from_host(&self, param_id: ParamID) -> tresult;
    unsafe fn end_edit_from_host(&self, param_id: ParamID) -> tresult;
}
