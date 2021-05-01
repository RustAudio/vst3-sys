use crate::base::{tchar, tresult};
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

pub enum NoteExpressionTypeID
{
	kVolumeTypeID = 0,
	kPanTypeID = 1,
	kTuningTypeID = 2,
	kVibratoTypeID = 3,
	kExpressionTypeID = 4,
	kBrightnessTypeID = 5,
	kTextTypeID = 6,
	kPhonemeTypeID = 7,
	kCustomStart = 100000,
	kCustomEnd = 200000,
	kInvalidTypeID = 0xFFFFFFFF
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct NoteExpressionValueDescription {
    pub default_value: f64,
    pub min: f64,
    pub max: f64,
    pub step_count: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct NoteExpressionValueEvent {
    pub type_id: NoteExpressionTypeID,
    pub note_id: i32,
    pub value: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NoteExpressionTextEvent {
    pub type_id: NoteExpressionTypeID,
    pub note_id: i32,
    pub text_len: u32,
    pub text: *const tchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NoteExpressionTypeInfo {
    pub type_id: NoteExpressionTypeID,
    pub title: [tchar; 128],
    pub short_title: [tchar; 128],
    pub units: [tchar; 128],
    pub unit_id: i32,
    pub value_desc: NoteExpressionValueDescription,
    pub id: u32,
    pub flags: i32,
}

pub enum KeySwitchTypeID
{
	kNoteOnKeyswitchTypeID = 0,
	kOnTheFlyKeyswitchTypeID = 1,	
	kOnReleaseKeyswitchTypeID = 2,
	kKeyRangeTypeID = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeySwitchInfo {
    pub type_id: KeySwitchTypeID,
    pub title: [tchar; 128],
    pub short_title: [tchar; 128],
    pub keyswitch_min: i32,
    pub keyswitch_max: i32,
    pub key_remapped: i32,
    pub unit_id: i32,
    pub flags: i32,
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
