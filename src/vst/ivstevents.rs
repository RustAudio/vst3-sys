use crate::base::tresult;
use crate::vst::{NoteExpressionTextEvent, NoteExpressionValueEvent};
use com::com_interface;
use com::interfaces::iunknown::IUnknown;

#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct NoteOnEvent {
    pub channel: i16,
    pub pitch: i16,
    pub tuning: f32,
    pub velocity: i32,
    pub length: i32,
    pub note_id: i32,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct NoteOffEvent {
    pub channel: i16,
    pub pitch: i16,
    pub velocity: i32,
    pub length: i32,
    pub note_id: i32,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct DataEvent {
    pub size: u32,
    pub type_: u32,
    pub bytes: *const u8,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct PolyPressureEvent {
    pub channel: i16,
    pub pitch: i16,
    pub pressure: f32,
    pub note_id: i32,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct ChordEvent {
    pub root: i16,
    pub bass_note: i16,
    pub mask: i16,
    pub text_len: u16,
    pub text: *const i16,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct ScaleEvent {
    pub root: i16,
    pub mask: i16,
    pub text_len: u16,
    pub text: *const i16,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct LegacyMidiCCOutEvent {
    pub control_number: u8,
    pub channel: i8,
    pub value: i8,
    pub value2: i8,
}

#[repr(align(16))]
#[derive(Copy, Clone)]
pub union EventData {
    note_on: NoteOnEvent,
    note_off: NoteOffEvent,
    data: DataEvent,
    poly: PolyPressureEvent,
    note_expression: NoteExpressionValueEvent,
    note_expression_text: NoteExpressionTextEvent,
    chord: ChordEvent,
    scale: ScaleEvent,
    midi_cc_out: LegacyMidiCCOutEvent,
}

#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct Event {
    pub bus_index: i32,
    pub sample_offset: i32,
    pub ppq_pos: f64,
    pub flags: u16,
    pub type_: u16,
    pub event: EventData,
}

#[com_interface("3A2C4214-3463-49FE-B2C4-F397B9695A44")]
pub trait IEventList: IUnknown {
    unsafe fn get_event_count(&self) -> i32;
    unsafe fn get_event(&self, idx: i32, event: *mut Event) -> tresult;
    unsafe fn add_event(&self, e: *mut Event) -> tresult;
}
