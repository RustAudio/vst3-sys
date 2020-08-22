#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct FrameRate {
    pub frames_per_second: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Chord {
    pub key_note: u8,
    pub root_note: u8,
    pub chord_mask: i16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ProcessContext {
    pub state: u32,
    pub sample_rate: f64,
    pub project_time_samples: i64,
    pub system_time: i64,
    pub continuous_time_samples: i64,
    pub project_time_music: f64,
    pub bar_position_music: f64,
    pub cycle_start_music: f64,
    pub cycle_end_music: f64,
    pub tempo: f64,
    pub time_sig_num: i32,
    pub time_sig_den: i32,
    pub chord: Chord,
    pub smpte_offset_subframes: i32,
    pub frame_rate: FrameRate,
    pub samples_to_next_clock: i64,
}
