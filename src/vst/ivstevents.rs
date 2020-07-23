use crate::base::tresult;
use crate::vst::{NoteExpressionTextEvent, NoteExpressionValueEvent};
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

pub enum EventTypes {
    kNoteOnEvent = 0,
    kNoteOffEvent = 1,
    kDataEvent = 2,
    kPolyPressureEvent = 3,
    kNoteExpressionValueEvent = 4,
    kNoteExpressionTextEvent = 5,
    kChordEvent = 6,
    kScaleEvent = 7,
    kLegacyMIDICCOutEvent = 65535,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct NoteOnEvent {
    pub channel: i16,
    pub pitch: i16,
    pub tuning: f32,
    pub velocity: i32,
    pub length: i32,
    pub note_id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct NoteOffEvent {
    pub channel: i16,
    pub pitch: i16,
    pub velocity: i32,
    pub length: i32,
    pub note_id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DataEvent {
    pub size: u32,
    pub type_: u32,
    pub bytes: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PolyPressureEvent {
    pub channel: i16,
    pub pitch: i16,
    pub pressure: f32,
    pub note_id: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ChordEvent {
    pub root: i16,
    pub bass_note: i16,
    pub mask: i16,
    pub text_len: u16,
    pub text: *const i16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ScaleEvent {
    pub root: i16,
    pub mask: i16,
    pub text_len: u16,
    pub text: *const i16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct LegacyMidiCCOutEvent {
    pub control_number: u8,
    pub channel: i8,
    pub value: i8,
    pub value2: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union EventData {
    pub note_on: NoteOnEvent,
    pub note_off: NoteOffEvent,
    pub data: DataEvent,
    pub poly_pressure: PolyPressureEvent,
    pub note_expression_value: NoteExpressionValueEvent,
    pub note_expression_text: NoteExpressionTextEvent,
    pub chord: ChordEvent,
    pub scale: ScaleEvent,
    pub legacy_midi_cc_out: LegacyMidiCCOutEvent,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Event {
    pub bus_index: i32,
    pub sample_offset: i32,
    pub ppq_position: f64,
    pub flags: u16,
    pub type_: u16,
    pub event: EventData,
}

#[com_interface("3A2C4214-3463-49FE-B2C4-F397B9695A44")]
pub trait IEventList: IUnknown {
    unsafe fn get_event_count(&self) -> i32;
    unsafe fn get_event(&self, index: i32, e: *mut Event) -> tresult;
    unsafe fn add_event(&self, e: *mut Event) -> tresult;
}
