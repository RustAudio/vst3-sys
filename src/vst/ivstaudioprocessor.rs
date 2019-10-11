use crate::vst::{IEventList, IParameterChanges, ProcessContext};
use com::{c_void, com_interface};

#[repr(align(16))]
#[derive(Copy, Clone, Default, Debug)]
pub struct ProcessSetup {
    process_mode: i32,
    symbolic_sample_size: i32,
    max_samples_per_block: i32,
    sample_rate: f64,
}

#[repr(align(16))]
#[derive(Debug)]
pub struct AudioBusBuffers {
    num_channels: i32,
    silence_flags: u64,
    buffers: *mut c_void,
}

#[repr(align(16))]
#[derive(Debug)]
pub struct ProcessData {
    process_mode: i32,
    symbolic_sample_size: i32,
    num_samples: i32,
    num_inputs: i32,
    num_outputs: i32,
    input: *mut AudioBusBuffers,
    output: *mut AudioBusBuffers,
    input_param_changes: *mut dyn IParameterChanges,
    output_param_changes: *mut dyn IParameterChanges,
    input_events: *mut dyn IEventList,
    output_events: *mut dyn IEventList,
    context: *mut ProcessContext,
}
