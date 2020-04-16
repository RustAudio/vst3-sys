use crate::base::tresult;
use crate::vst::{NoteExpressionTextEvent, NoteExpressionValueEvent};
use com::com_interface;
use com::interfaces::iunknown::IUnknown;

#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct NoteOnEvent {
    channel: i16,
    pitch: i16,
    tuning: f32,
    velocity: i32,
    length: i32,
    note_id: i32,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct NoteOffEvent {
    channel: i16,
    pitch: i16,
    velocity: i32,
    length: i32,
    note_id: i32,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct DataEvent {
    size: u32,
    type_: u32,
    bytes: *const u8,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct PolyPressureEvent {
    channel: i16,
    pitch: i16,
    pressure: f32,
    note_id: i32,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct ChordEvent {
    root: i16,
    bass_note: i16,
    mask: i16,
    text_len: u16,
    text: *const i16,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct ScaleEvent {
    root: i16,
    mask: i16,
    text_len: u16,
    text: *const i16,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct LegacyMidiCCOutEvent {
    control_number: u8,
    channel: i8,
    value: i8,
    value2: i8,
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
    bus_index: i32,
    sample_offset: i32,
    ppq_pos: f64,
    flags: u16,
    type_: u16,
    event: EventData,
}

#[com_interface("3A2C4214-3463-49FE-B2C4-F397B9695A44")]
pub trait IEventList: IUnknown {
    unsafe fn get_event_count(&self) -> i32;
    unsafe fn get_event(&self, idx: i32, event: *mut Event) -> tresult;
    unsafe fn add_event(&self, e: *mut Event) -> tresult;
}
