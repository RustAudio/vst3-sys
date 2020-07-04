use crate::vst::{Speaker, SpeakerArrangement};

pub const kSpeakerL: Speaker = 1 << 0;
pub const kSpeakerR: Speaker = 1 << 1;
// todo: add other Speakers
pub const kSpeakerM: Speaker = 1 << 19;
// todo: add other Speakers

pub const kEmpty: SpeakerArrangement = 0;
pub const kMono: SpeakerArrangement = kSpeakerM;
pub const kStereo: SpeakerArrangement = kSpeakerL | kSpeakerR;
// todo: add other SpeakerArrangements
