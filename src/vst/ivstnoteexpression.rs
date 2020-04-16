use crate::base::{tchar, tresult};
use com::com_interface;
use com::interfaces::iunknown::IUnknown;

#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct NoteExpressionValueDescription {
    default_value: f64,
    min: f64,
    max: f64,
    step_count: i32,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct NoteExpressionValueEvent {
    type_id: u32,
    note_id: i32,
    value: f64,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct NoteExpressionTextEvent {
    type_id: u32,
    note_id: i32,
    text_len: u32,
    text: *const tchar,
}

#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct NoteExpressionTypeInfo {
    type_id: u32,
    title: [tchar; 128],
    short_title: [tchar; 128],
    units: [tchar; 128],
    unit_id: i32,
    value_desc: NoteExpressionValueDescription,
    id: u32,
    flags: i32,
}

#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct KeySwitchInfo {
    type_id: u32,
    title: [tchar; 128],
    short_title: [tchar; 128],
    keyswitch_min: i32,
    keyswitch_max: i32,
    key_remapped: i32,
    unit_id: i32,
    flags: i32,
}

#[com_interface("B7F8F859-4123-4872-9116-95814F3721A3")]
pub trait INoteExpressionController: IUnknown {
    unsafe fn get_note_expression_count(&self, bus_idx: i32, channel: i16) -> i32;
    unsafe fn get_note_expression_info(
        &self,
        bus_idx: i32,
        channel: i16,
        note_expression_idx: i32,
        info: *mut NoteExpressionTypeInfo,
    ) -> tresult;
    unsafe fn get_note_expression_string_by_value(
        &self,
        bus_idx: i32,
        channel: i16,
        id: u32,
        value: f64,
        string: *mut tchar,
    ) -> tresult;
    unsafe fn get_note_expression_value_by_string(
        &self,
        bus_idx: i32,
        channel: i16,
        id: u32,
        string: *const tchar,
        value: *mut f64,
    ) -> tresult;
}

#[com_interface("1F2F76D3-BFFB-4B96-B995-27A55EBCCEF4")]
pub trait IKeyswitchController: IUnknown {
    unsafe fn get_keyswitch_count(&self, bus_index: i32, channel: i16) -> i32;
    unsafe fn get_keyswitch_info(
        &self,
        bus_inex: i32,
        channel: i16,
        keyswitch_idx: i32,
        info: *mut KeySwitchInfo,
    ) -> tresult;
}
