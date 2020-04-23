use crate::base::{tchar, tresult, IBStream};
use crate::vst::{CString, String128};
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;
//todo: ivstunit constants

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UnitInfo {
    pub id: i32,
    pub parent_unit_id: i32,
    pub name: String128,
    pub program_list_id: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProgramListInfo {
    pub id: i32,
    pub name: String128,
    pub program_count: i32,
}

#[com_interface("4B5147F8-4654-486B-8DAB-30BA163A3C56")]
pub trait IUnitHandler: IUnknown {
    unsafe fn notify_unit_selection(&self, unit_id: i32) -> tresult;
    unsafe fn notify_program_list_change(&self, list_id: i32, program_idx: i32) -> tresult;
}

#[com_interface("F89F8CDF-699E-4BA5-96AA-C9A481452B01")]
pub trait IUnitHandler2: IUnitHandler {
    unsafe fn notify_unit_by_bus_change(&self) -> tresult;
}

// todo: update types
#[com_interface("3D4BD6B5-913A-4FD2-A886-E768A5EB92C1")]
pub trait IUnitInfo: IUnknown {
    unsafe fn get_unit_count(&self) -> i32;
    unsafe fn get_unit_info(&self, unit_index: i32, info: *mut UnitInfo) -> tresult;
    unsafe fn get_program_list_count(&self) -> i32;
    unsafe fn get_program_list_info(&self, list_index: i32, info: *mut ProgramListInfo) -> tresult;
    unsafe fn get_program_name(&self, list_id: i32, program_index: i32, name: String128)
        -> tresult;
    unsafe fn get_program_info(
        &self,
        list_id: i32,
        program_index: i32,
        attribute_id: CString,
        attribute_value: String128,
    ) -> tresult;
    unsafe fn has_program_pitch_names(&self, id: i32, idx: i32) -> tresult;
    unsafe fn get_program_pitch_name(
        &self,
        id: i32,
        idx: i32,
        pitch: i16,
        name: *mut tchar,
    ) -> tresult;
    unsafe fn get_selected_unit(&self) -> i32;
    unsafe fn select_unit(&self, id: i32) -> tresult;
    unsafe fn get_unit_by_bus(
        &self,
        type_: i32,
        dir: i32,
        idx: i32,
        channel: i32,
        unit_id: *mut i32,
    ) -> tresult;
    unsafe fn set_unit_program_data(
        &self,
        list_or_unit: i32,
        program_idx: i32,
        data: *mut dyn IBStream,
    ) -> tresult;
}

#[com_interface("8683B01F-7B35-4F70-A265-1DEC353AF4FF")]
pub trait IProgramListData: IUnknown {
    unsafe fn program_data_supported(&self, id: i32) -> tresult;
    unsafe fn get_program_data(
        &self,
        list_id: i32,
        program_idx: i32,
        stream: *mut dyn IBStream,
    ) -> tresult;
    unsafe fn set_program_data(&self, id: i32, idx: i32, stream: *mut dyn IBStream) -> tresult;
}

#[com_interface("6C389611-D391-455D-B870-B83394A0EFDD")]
pub trait IUnitData: IUnknown {
    unsafe fn unit_data_supported(&self, id: i32) -> tresult;
    unsafe fn get_unit_data(&self, id: i32, data: *mut dyn IBStream) -> tresult;
    unsafe fn set_unit_data(&self, id: i32, data: *mut dyn IBStream) -> tresult;
}
