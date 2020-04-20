#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct FrameRate {
    frames_per_second: u32,
    flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Chord {
    key_note: u8,
    root_note: u8,
    chord_mask: i16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ProcessContext {
    state: u32,
    sample_rate: f64,
    project_time_samples: i64,
    system_time: i64,
    continuous_time_samples: i64,
    project_time_music: f64,
    bar_position_music: f64,
    cycle_start_music: f64,
    cycle_end_music: f64,
    chord: Chord,
    smpte_offset_subframes: i32,
    frame_rate: FrameRate,
    samples_to_next_clock: i64,
}
