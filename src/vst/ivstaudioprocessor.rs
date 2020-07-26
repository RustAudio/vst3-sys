use crate::base::{tresult, TBool};
use crate::utils::VstPtr;
use crate::vst::{
    BusDirection, CString, IEventList, IParameterChanges, ProcessContext, SpeakerArrangement,
};
use vst3_com::interfaces::iunknown::IUnknown;
use vst3_com::{c_void, com_interface};

pub const kVstAudioEffectClass: CString = b"Audio Module Class\0".as_ptr() as *const _;

/// todo: add all subcategories
pub const kFx: CString = b"Fx\0".as_ptr() as *const _;

pub const K_SAMPLE32: i32 = 0;
pub const K_SAMPLE64: i32 = 1;

pub enum ProcessModes {
    kRealtime = 0,
    kPrefetch = 1,
    kOffline = 2,
}

pub enum SymbolicSampleSizes {
    kSample32 = 0,
    kSample64 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ProcessSetup {
    pub process_mode: i32,
    pub symbolic_sample_size: i32,
    pub max_samples_per_block: i32,
    pub sample_rate: f64,
}

#[repr(C)]
#[derive(Debug)]
pub struct AudioBusBuffers {
    pub num_channels: i32,
    pub silence_flags: u64,
    pub buffers: *mut *mut c_void,
}

#[repr(C)]
pub struct ProcessData {
    pub process_mode: i32,
    pub symbolic_sample_size: i32,
    pub num_samples: i32,
    pub num_inputs: i32,
    pub num_outputs: i32,
    pub inputs: *mut AudioBusBuffers,
    pub outputs: *mut AudioBusBuffers,
    pub input_param_changes: VstPtr<dyn IParameterChanges>,
    pub output_param_changes: VstPtr<dyn IParameterChanges>,
    pub input_events: VstPtr<dyn IEventList>,
    pub output_events: VstPtr<dyn IEventList>,
    pub context: *mut ProcessContext,
}

#[com_interface("42043F99-B7DA-453C-A569-E79D9AAEC33D")]
pub trait IAudioProcessor: IUnknown {
    unsafe fn set_bus_arrangements(
        &self,
        inputs: *mut SpeakerArrangement,
        num_ins: i32,
        outputs: *mut SpeakerArrangement,
        num_outs: i32,
    ) -> tresult;
    unsafe fn get_bus_arrangement(
        &self,
        dir: BusDirection,
        index: i32,
        arr: *mut SpeakerArrangement,
    ) -> tresult;
    unsafe fn can_process_sample_size(&self, symbolic_sample_size: i32) -> tresult;
    unsafe fn get_latency_samples(&self) -> u32;
    unsafe fn setup_processing(&self, setup: *const ProcessSetup) -> tresult;
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
