//! Author: Mike Hilgendorf <mike@hilgendorf.audio>
//!
//! Bare minimum plugin that copies input to output, doesn't
//! save its own state, and doesn't have any parameters.
#![allow(clippy::collapsible_if)]
#![allow(clippy::missing_safety_doc)]
use log::*;
use std::{
    os::raw::{c_char, c_short, c_void},
    ptr::{copy_nonoverlapping, null_mut},
};
use vst3_com::{sys::GUID, IID};
use vst3_sys::{
    base::{
        kInvalidArgument, kResultFalse, kResultOk, tresult, FIDString, IPluginBase, IPluginFactory,
        TBool,
    },
    vst::{
        AudioBusBuffers, BusDirection, BusDirections, BusFlags, BusInfo, IAudioPresentationLatency,
        IAudioProcessor, IAutomationState, IComponent, IEditController, MediaTypes, ParameterInfo,
        ProcessData, ProcessSetup, RoutingInfo, TChar,
    },
    VST3,
};
use widestring::U16CString;

unsafe fn strcpy(src: &str, dst: *mut c_char) {
    copy_nonoverlapping(src.as_ptr() as *const c_void as *const _, dst, src.len());
}

unsafe fn wstrcpy(src: &str, dst: *mut c_short) {
    let src = U16CString::from_str(src).unwrap();
    let mut src = src.into_vec();
    src.push(0);
    copy_nonoverlapping(src.as_ptr() as *const c_void as *const _, dst, src.len());
}

#[VST3(implements(
    IComponent,
    IEditController,
    IAudioProcessor,
    IAutomationState,
    IAudioPresentationLatency
))]
pub struct PassthruPlugin {}
pub struct PassthruController {}
impl PassthruPlugin {
    const CID: GUID = GUID {
        data: [
            0x93, 0x68, 0x4f, 0x1a, 0x46, 0x11, 0x91, 0x01, 0x00, 0x00, 0xb4, 0x39, 0xe5, 0x64,
            0x8a, 0xda,
        ],
    };
    pub fn new() -> Box<Self> {
        PassthruPlugin::allocate()
    }
}

#[VST3(implements(IPluginFactory))]
pub struct Factory {}

impl IEditController for PassthruPlugin {
    unsafe fn set_component_state(&mut self, _state: *mut c_void) -> tresult {
        info!("set_component_state");
        kResultOk
    }
    unsafe fn set_state(&mut self, _state: *mut c_void) -> tresult {
        info!("set_state");
        kResultOk
    }
    unsafe fn get_state(&mut self, _state: *mut c_void) -> tresult {
        info!("get_state");
        kResultOk
    }
    unsafe fn get_parameter_count(&self) -> i32 {
        info!("get_parameter_count");
        0
    }
    unsafe fn get_parameter_info(&self, _: i32, _: *mut ParameterInfo) -> tresult {
        info!("get_parameter_info");
        kResultFalse
    }
    unsafe fn get_param_string_by_value(
        &self,
        _id: u32,
        _value_normalized: f64,
        _string: *mut TChar,
    ) -> tresult {
        info!("get_param_string_by_value");
        kResultFalse
    }
    unsafe fn get_param_value_by_string(
        &self,
        _id: u32,
        _string: *mut TChar,
        _value_normalized: *mut f64,
    ) -> tresult {
        info!("get_param_value_by_string");
        kResultFalse
    }
    unsafe fn normalized_param_to_plain(&self, _id: u32, _value_normalized: f64) -> f64 {
        info!("normalized_param_to_plain");
        0.0
    }
    unsafe fn plain_param_to_normalized(&self, _id: u32, _plain_value: f64) -> f64 {
        info!("plain_param_to_normalized");
        0.0
    }
    unsafe fn get_param_normalized(&self, _id: u32) -> f64 {
        info!("get_param_normalized");
        0.0
    }
    unsafe fn set_param_normalized(&mut self, _id: u32, _value: f64) -> tresult {
        info!("set_param_normalized");
        kResultOk
    }
    unsafe fn set_component_handler(&mut self, _handler: *mut c_void) -> tresult {
        info!("set_component_handler");
        kResultOk
    }
    unsafe fn create_view(&self, _name: FIDString) -> *mut c_void {
        info!("Called: AGainController::create_view()");
        null_mut()
    }
}
impl IAudioProcessor for PassthruPlugin {
    unsafe fn set_bus_arrangements(
        &self,
        _inputs: *mut u64,
        _num_ins: i32,
        _outputs: *mut u64,
        _num_outs: i32,
    ) -> i32 {
        kResultFalse
    }

    unsafe fn get_bus_arrangements(&self, dir: i32, idx: i32, arr: *mut u64) -> i32 {
        info!("get_bus(): dir: {}, idx: {}, arr: {:016b}", dir, idx, *arr);
        let arr = &mut *arr;
        if (*arr == 0x0) || (*arr == 0x1) || (*arr == 0x3) {
            kResultOk
        } else {
            *arr = 0x03;
            kResultOk
        }
    }

    unsafe fn can_process_sample_size(&self, _symbolic_sample_size: i32) -> i32 {
        kResultOk
    }

    unsafe fn get_latency_sample(&self) -> u32 {
        0
    }
    unsafe fn setup_processing(&mut self, _setup: *mut ProcessSetup) -> tresult {
        kResultOk
    }
    unsafe fn set_processing(&self, _state: TBool) -> tresult {
        kResultOk
    }
    unsafe fn process(&mut self, data: *mut ProcessData) -> tresult {
        let data = &*data;
        let num_samples = data.num_samples as usize;
        if data.inputs.is_null() || data.outputs.is_null() {
            return kResultOk;
        }
        let inputs: &mut AudioBusBuffers = &mut *data.inputs;
        let outputs: &mut AudioBusBuffers = &mut *data.outputs;
        let num_channels = inputs.num_channels as usize;
        let input_ptr = std::slice::from_raw_parts(inputs.buffers, num_channels);
        let output_ptr = std::slice::from_raw_parts(outputs.buffers, num_channels);
        let sample_size = if data.symbolic_sample_size == 0 { 4 } else { 8 };
        for (i, o) in input_ptr.iter().zip(output_ptr.iter()) {
            copy_nonoverlapping(*i, *o, num_samples * sample_size);
        }
        kResultOk
    }
    unsafe fn get_tail_samples(&self) -> u32 {
        0
    }
}

impl IAudioPresentationLatency for PassthruPlugin {
    unsafe fn set_audio_presentation_latency_sample(
        &self,
        _dir: BusDirection,
        _bus_idx: i32,
        _latency_samples: u32,
    ) -> tresult {
        kResultOk
    }
}

impl IAutomationState for PassthruPlugin {
    unsafe fn set_automation_state(&self, _state: i32) -> tresult {
        kResultOk
    }
}

impl IPluginBase for PassthruPlugin {
    unsafe fn initialize(&mut self, _host_context: *mut c_void) -> tresult {
        kResultOk
    }
    unsafe fn terminate(&mut self) -> tresult {
        kResultOk
    }
}

impl IComponent for PassthruPlugin {
    unsafe fn get_controller_class_id(&self, _tuid: *mut IID) -> tresult {
        kResultOk
    }

    unsafe fn set_io_mode(&self, _mode: i32) -> tresult {
        kResultOk
    }

    unsafe fn get_bus_count(&self, type_: i32, _dir: i32) -> i32 {
        if type_ == MediaTypes::kAudio as i32 {
            1
        } else {
            0
        }
    }

    unsafe fn get_bus_info(&self, type_: i32, dir: i32, _idx: i32, info: *mut BusInfo) -> tresult {
        if type_ == MediaTypes::kAudio as i32 {
            let info = &mut *info;
            if dir == BusDirections::kInput as i32 {
                info.direction = dir;
                info.bus_type = MediaTypes::kAudio as i32;
                info.channel_count = 2;
                info.flags = BusFlags::kDefaultActive as u32;
                wstrcpy("Audio Input", info.name.as_mut_ptr());
            } else {
                info.direction = dir;
                info.bus_type = MediaTypes::kAudio as i32;
                info.channel_count = 2;
                info.flags = BusFlags::kDefaultActive as u32;
                wstrcpy("Audio Output", info.name.as_mut_ptr());
            }
            kResultOk
        } else {
            kInvalidArgument
        }
    }

    unsafe fn get_routing_info(
        &self,
        _in_info: *mut RoutingInfo,
        _out_info: *mut RoutingInfo,
    ) -> i32 {
        kResultFalse
    }

    unsafe fn activate_bus(&mut self, _type_: i32, _dir: i32, _idx: i32, _state: TBool) -> tresult {
        kResultOk
    }

    unsafe fn set_active(&self, _state: TBool) -> tresult {
        kResultOk
    }

    unsafe fn set_state(&mut self, _state: *mut c_void) -> tresult {
        kResultOk
    }

    unsafe fn get_state(&mut self, _state: *mut c_void) -> tresult {
        kResultOk
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Factory implementation

impl Factory {
    fn new() -> Box<Self> {
        info!("instantiating factory...");
        Self::allocate()
    }
}

impl IPluginFactory for Factory {
    unsafe fn get_factory_info(&self, info: *mut vst3_sys::base::PFactoryInfo) -> i32 {
        let info = &mut *info;
        strcpy("rust.audio", info.vendor.as_mut_ptr());
        strcpy("https://rust.audio", info.url.as_mut_ptr());
        strcpy("mailto://mike@hilgendorf.audio", info.email.as_mut_ptr());
        info.flags = 8;
        kResultOk
    }

    unsafe fn count_classes(&self) -> i32 {
        1
    }
    unsafe fn get_class_info(&self, idx: i32, info: *mut vst3_sys::base::PClassInfo) -> i32 {
        match idx {
            0 => {
                let info = &mut *info;
                info.cardinality = 0x7FFF_FFFF;
                info.cid = PassthruPlugin::CID;
                strcpy("Audio Module Class", info.category.as_mut_ptr());
                strcpy("Pass Through", info.name.as_mut_ptr());
            }
            _ => {
                info!("invalid class info ID {}", idx);
                return kInvalidArgument;
            }
        }
        kResultOk
    }
    unsafe fn create_instance(
        &self,
        cid: *mut vst3_com::sys::GUID,
        _riid: *mut vst3_com::sys::GUID,
        obj: *mut *mut core::ffi::c_void,
    ) -> i32 {
        let iid = *cid;
        match iid {
            PassthruPlugin::CID => {
                let ptr = Box::into_raw(PassthruPlugin::new()) as *mut c_void;
                *obj = ptr;
                kResultOk
            }
            _ => kResultFalse,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn init() {
    if let Err(e) = simple_logger::init() {
        println!("{:?}", e);
    }
    info!("plugin library loaded");
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn GetPluginFactory() -> *mut c_void {
    info!("calling plugin factory");
    Box::into_raw(Factory::new()) as *mut c_void
}

#[cfg(target_os = "linux")]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn ModuleEntry(_: *mut c_void) -> bool {
    init();
    true
}

#[cfg(target_os = "linux")]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn ModuleExit() -> bool {
    true
}

#[cfg(target_os = "macos")]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn bundleEntry() -> bool {
    init();
    true
}

#[cfg(target_os = "linux")]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn bundleExit() -> bool {
    true
}

#[cfg(target_os = "windows")]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn InitDll() -> bool {
    init();
    true
}

#[cfg(target_os = "windows")]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn ExitDll() -> bool {
    true
}
