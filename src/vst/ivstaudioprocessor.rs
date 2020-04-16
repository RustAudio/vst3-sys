use crate::base::{tresult, TBool};
use crate::vst::{BusDirection, IEventList, IParameterChanges, ProcessContext};
use com::interfaces::iunknown::IUnknown;
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

#[com_interface("42043F99-B7DA-453C-A569-E79D9AAEC33D")]
pub trait IAudioProcessor: IUnknown {
    unsafe fn get_latency_sample(&self) -> u32;
    unsafe fn setup_processing(&self, setup: *mut ProcessSetup) -> tresult;
    unsafe fn set_processing(&self, state: TBool) -> tresult;
    unsafe fn process(&self, data: *mut ProcessData) -> tresult;
    unsafe fn get_tail_samples(&self) -> u32;
}

#[com_interface("309ECE78-EB7D-4fae-8B22-25D909FD08B6")]
pub trait IAudioPresentationLatency: IUnknown {
    unsafe fn set_audio_presentation_latency_sample(
        &self,
        dir: BusDirection,
        bus_idx: i32,
        latency_samples: u32,
    ) -> tresult;
}
