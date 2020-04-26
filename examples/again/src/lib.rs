use std::os::raw::{c_char, c_short, c_void};
use std::ptr::{copy_nonoverlapping, null_mut};

use log::*;

use flexi_logger::{opt_format, Logger};
use std::intrinsics::write_bytes;
use std::mem;
use vst3_com::sys::GUID;
use vst3_com::{ComPtr, IID};
use vst3_sys::base::{
    kInvalidArgument, kNotImplemented, kResultFalse, kResultOk, kResultTrue, tresult,
    ClassCardinality, FIDString, IBStream, IPluginBase, IPluginFactory, IPluginFactory2, IUnknown,
    PClassInfo, PClassInfo2, PFactoryInfo, TBool,
};
use vst3_sys::vst::ParameterFlags::{kCanAutomate, kIsBypass};
use vst3_sys::vst::{
    BusDirection, BusInfo, BusType, IAudioProcessor, IComponent, IComponentHandler,
    IEditController, IParamValueQueue, IParameterChanges, IUnitInfo, IoMode, MediaType,
    ParameterInfo, ProcessData, ProcessSetup, ProgramListInfo, RoutingInfo, SpeakerArrangement,
    TChar, UnitInfo, K_SAMPLE32, K_SAMPLE64,
};
use vst3_sys::VST3;
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

pub struct AudioBus {
    name: String,
    bus_type: BusType,
    flags: i32,
    active: TBool,
    speaker_arr: SpeakerArrangement,
}

#[VST3(implements(IComponent, IAudioProcessor))]
pub struct AGainProcessor {
    current_process_mode: i32,
    process_setup: ProcessSetup,
    audio_inputs: Vec<AudioBus>,
    audio_outputs: Vec<AudioBus>,
    gain: f64,
    bypass: bool,
    context: *mut c_void,
}
impl AGainProcessor {
    const CID: GUID = GUID {
        data: [
            0x84, 0xE8, 0xDE, 0x5F, 0x92, 0x55, 0x4F, 0x53, 0x96, 0xFA, 0xE4, 0x13, 0x3C, 0x93,
            0x5A, 0x18,
        ],
    };

    pub fn new() -> Box<Self> {
        let current_process_mode = 0;
        let process_setup = ProcessSetup {
            process_mode: 0,
            symbolic_sample_size: 0,
            max_samples_per_block: 0,
            sample_rate: 0.0,
        };
        let audio_inputs = vec![];
        let audio_outputs = vec![];
        let gain = 1.0;
        let bypass = false;
        let context = null_mut();
        AGainProcessor::allocate(
            current_process_mode,
            process_setup,
            audio_inputs,
            audio_outputs,
            gain,
            bypass,
            context,
        )
    }

    pub fn create_instance() -> *mut c_void {
        Box::into_raw(Self::new()) as *mut c_void
    }

    pub unsafe fn setup_processing_ae(&mut self, new_setup: *mut ProcessSetup) -> tresult {
        if self.can_process_sample_size((*new_setup).symbolic_sample_size) != kResultTrue {
            return kResultFalse;
        }
        self.process_setup = (*new_setup).clone();
        kResultOk
    }

    pub unsafe fn add_audio_input(&mut self, name: &str, arr: SpeakerArrangement) {
        let new_bus = AudioBus {
            name: String::from(name),
            bus_type: 0,
            flags: 1,
            active: false as u8,
            speaker_arr: arr,
        };
        self.audio_inputs.push(new_bus);
    }

    pub unsafe fn add_audio_output(&mut self, name: &str, arr: SpeakerArrangement) {
        let new_bus = AudioBus {
            name: String::from(name),
            bus_type: 0,
            flags: 1,
            active: false as u8,
            speaker_arr: arr,
        };
        self.audio_outputs.push(new_bus);
    }
}

fn get_channel_count(arr: SpeakerArrangement) -> i32 {
    let mut arr = arr;
    let mut count = 0;
    while arr != 0 {
        if (arr & 1) == 1 {
            count += 1;
        }
        arr >>= 1;
    }
    count
}

impl IComponent for AGainProcessor {
    unsafe fn get_controller_class_id(&self, tuid: *mut IID) -> tresult {
        info!("Called: AGainProcessor::get_controller_class_id()");

        *tuid = AGainController::CID.clone();
        kResultOk
    }

    unsafe fn set_io_mode(&self, _mode: IoMode) -> tresult {
        info!("Called: AGainProcessor::set_io_mode()");

        kNotImplemented
    }

    unsafe fn get_bus_count(&self, type_: MediaType, dir: BusDirection) -> i32 {
        info!("Called: AGainProcessor::get_bus_count()");

        match type_ {
            0 => match dir {
                0 => self.audio_inputs.len() as i32,
                _ => self.audio_outputs.len() as i32,
            },
            _ => 0,
        }
    }

    unsafe fn get_bus_info(
        &self,
        type_: MediaType,
        dir: BusDirection,
        index: i32,
        info: *mut BusInfo,
    ) -> tresult {
        info!("Called: AGainProcessor::get_bus_info()");

        (*info).media_type = type_;
        (*info).direction = dir;
        match type_ {
            0 => match dir {
                0 => {
                    if index as usize >= self.audio_inputs.len() {
                        kInvalidArgument
                    } else {
                        let bus = &self.audio_inputs[index as usize];
                        wstrcpy(&bus.name, (*info).name.as_mut_ptr());
                        (*info).channel_count = get_channel_count(bus.speaker_arr);
                        (*info).bus_type = bus.bus_type;
                        (*info).flags = bus.flags as u32;
                        kResultTrue
                    }
                }
                _ => {
                    if index as usize >= self.audio_outputs.len() {
                        kInvalidArgument
                    } else {
                        let bus = &self.audio_outputs[index as usize];
                        wstrcpy(&bus.name, (*info).name.as_mut_ptr());
                        (*info).channel_count = get_channel_count(bus.speaker_arr);
                        (*info).bus_type = bus.bus_type;
                        (*info).flags = bus.flags as u32;
                        kResultTrue
                    }
                }
            },
            _ => kResultFalse,
        }
    }

    unsafe fn get_routing_info(
        &self,
        _in_info: *mut RoutingInfo,
        _out_info: *mut RoutingInfo,
    ) -> tresult {
        info!("Called: AGainProcessor::get_routing_info()");

        kNotImplemented
    }

    unsafe fn activate_bus(
        &mut self,
        type_: MediaType,
        dir: BusDirection,
        index: i32,
        state: TBool,
    ) -> tresult {
        info!("Called: AGainProcessor::activate_bus()");

        if index < 0
            || index >= self.audio_inputs.len() as i32
            || index >= self.audio_outputs.len() as i32
        {
            return kInvalidArgument;
        }

        match type_ {
            0 => match dir {
                0 => {
                    self.audio_inputs[index as usize].active = state;
                }
                _ => {
                    self.audio_outputs[index as usize].active = state;
                }
            },
            _ => return kInvalidArgument,
        }
        kResultTrue
    }

    unsafe fn set_active(&self, _state: TBool) -> tresult {
        info!("Called: AGainProcessor::set_active()");

        kResultOk
    }

    unsafe fn set_state(&mut self, state: *mut c_void) -> tresult {
        info!("Called: AGainProcessor::set_state()");

        if state.is_null() {
            return kResultFalse;
        }

        let state = state as *mut *mut _;
        let state: ComPtr<dyn IBStream> = ComPtr::new(state);

        let mut num_bytes_read = 0;
        let mut saved_gain = 0.0;
        let mut saved_bypass = false;
        let gain_ptr = &mut saved_gain as *mut f64 as *mut c_void;
        let bypass_ptr = &mut saved_bypass as *mut bool as *mut c_void;

        state.read(gain_ptr, mem::size_of::<f64>() as i32, &mut num_bytes_read);
        state.read(
            bypass_ptr,
            mem::size_of::<bool>() as i32,
            &mut num_bytes_read,
        );

        self.gain = saved_gain;
        self.bypass = saved_bypass;

        kResultOk
    }

    unsafe fn get_state(&mut self, state: *mut c_void) -> tresult {
        info!("Called: AGainProcessor::get_state()");

        if state.is_null() {
            return kResultFalse;
        }

        let state = state as *mut *mut _;
        let state: ComPtr<dyn IBStream> = ComPtr::new(state);

        let mut num_bytes_written = 0;
        let gain_ptr = &mut self.gain as *mut f64 as *mut c_void;
        let bypass_ptr = &mut self.bypass as *mut bool as *mut c_void;

        state.write(
            gain_ptr,
            mem::size_of::<f64>() as i32,
            &mut num_bytes_written,
        );
        state.write(
            bypass_ptr,
            mem::size_of::<bool>() as i32,
            &mut num_bytes_written,
        );

        kResultOk
    }
}

impl IPluginBase for AGainProcessor {
    unsafe fn initialize(&mut self, context: *mut c_void) -> tresult {
        info!("Called: AGainProcessor::initialize()");

        if !self.context.is_null() {
            return kResultFalse;
        }
        self.context = context;

        self.add_audio_input("Stereo In", 3);
        self.add_audio_output("Stereo Out", 3);

        kResultOk
    }
    unsafe fn terminate(&mut self) -> tresult {
        info!("Called: AGainProcessor::terminate()");

        self.audio_inputs.clear();
        self.audio_outputs.clear();
        self.context = null_mut();
        kResultOk
    }
}

impl IAudioProcessor for AGainProcessor {
    unsafe fn set_bus_arrangements(
        &self,
        _inputs: *mut SpeakerArrangement,
        _num_ins: i32,
        _outputs: *mut SpeakerArrangement,
        _num_outs: i32,
    ) -> tresult {
        info!("Called: AGainProcessor::set_bus_arrangements()");

        kResultFalse
    }
    unsafe fn get_bus_arrangements(
        &self,
        dir: BusDirection,
        index: i32,
        arr: *mut SpeakerArrangement,
    ) -> tresult {
        info!("Called: AGainProcessor::get_bus_arrangements()");

        match dir {
            0 => {
                if index as usize >= self.audio_inputs.len() {
                    kResultFalse
                } else {
                    *arr = self.audio_inputs[index as usize].speaker_arr;
                    kResultTrue
                }
            }
            _ => {
                if index as usize >= self.audio_outputs.len() {
                    kResultFalse
                } else {
                    *arr = self.audio_outputs[index as usize].speaker_arr;
                    kResultTrue
                }
            }
        }
    }
    unsafe fn can_process_sample_size(&self, symbolic_sample_size: i32) -> tresult {
        info!("Called: AGainProcessor::can_process_sample_size()");

        match symbolic_sample_size {
            K_SAMPLE32 => kResultTrue,
            K_SAMPLE64 => kResultTrue,
            _ => kResultFalse,
        }
    }
    unsafe fn get_latency_sample(&self) -> u32 {
        info!("Called: AGainProcessor::get_latency_sample()");

        0
    }
    unsafe fn setup_processing(&mut self, setup: *mut ProcessSetup) -> tresult {
        info!("Called: AGainProcessor::setup_processing()");

        self.current_process_mode = (*setup).process_mode;
        self.setup_processing_ae(setup)
    }
    unsafe fn set_processing(&self, _state: TBool) -> tresult {
        info!("Called: AGainProcessor::set_processing()");

        kNotImplemented
    }
    unsafe fn process(&mut self, data: *mut ProcessData) -> tresult {
        info!("Called: AGainProcessor::process()");

        let param_changes = (*data).input_param_changes as *mut c_void;
        if !param_changes.is_null() {
            let param_changes = param_changes as *mut *mut _;
            let param_changes: ComPtr<dyn IParameterChanges> = ComPtr::new(param_changes);
            let num_params_changed = param_changes.get_parameter_count();
            for i in 0..num_params_changed {
                let param_queue = param_changes.get_parameter_data(i);
                if !param_queue.is_null() {
                    let param_queue = param_queue as *mut *mut _;
                    let param_queue: ComPtr<dyn IParamValueQueue> = ComPtr::new(param_queue);
                    let mut value = 0.0;
                    let mut sample_offset = 0;
                    let num_points = param_queue.get_point_count();
                    match param_queue.get_parameter_id() {
                        0 => {
                            if param_queue.get_point(
                                num_points - 1,
                                &mut sample_offset as *mut _,
                                &mut value as *mut _,
                            ) == kResultTrue
                            {
                                self.gain = value;
                                info!("Gain value: {}", self.gain);
                            }
                        }
                        1 => {
                            if param_queue.get_point(
                                num_points - 1,
                                &mut sample_offset as *mut _,
                                &mut value as *mut _,
                            ) == kResultTrue
                            {
                                self.bypass = value > 0.5;
                                info!("Bypass value: {}", self.bypass);
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        if (*data).num_inputs == 0 && (*data).num_outputs == 0 {
            return kResultOk;
        }

        let num_channels = (*(*data).inputs).num_channels;
        let num_samples = (*data).num_samples;
        let in_ = (*(*data).inputs).buffers;
        let out_ = (*(*data).outputs).buffers;
        let sample_frames_size = {
            match self.process_setup.symbolic_sample_size {
                K_SAMPLE32 => (*data).num_samples as usize * mem::size_of::<f32>(),
                K_SAMPLE64 => (*data).num_samples as usize * mem::size_of::<f64>(),
                _ => unreachable!(),
            }
        };

        if (*(*data).inputs).silence_flags != 0 {
            (*(*data).outputs).silence_flags = (*(*data).inputs).silence_flags;
            for i in 0..num_channels as isize {
                write_bytes(*out_.offset(i), 0, sample_frames_size);
            }
            return kResultOk;
        }

        (*(*data).outputs).silence_flags = 0;

        if self.bypass {
            for i in 0..num_channels as isize {
                if *in_.offset(i) != *out_.offset(i) {
                    copy_nonoverlapping(
                        *in_.offset(i) as *const c_void,
                        *out_.offset(i),
                        sample_frames_size,
                    );
                }
            }
        } else {
            match self.process_setup.symbolic_sample_size {
                K_SAMPLE32 => {
                    info!("Processing at 32bit");
                    for i in 0..num_channels as isize {
                        let channel_in = *in_.offset(i) as *const f32;
                        let channel_out = *out_.offset(i) as *mut f32;
                        for j in 0..num_samples as isize {
                            *channel_out.offset(j) = *channel_in.offset(j) * self.gain as f32;
                        }
                    }
                }
                K_SAMPLE64 => {
                    info!("Processing at 64bit");
                    for i in 0..num_channels as isize {
                        let channel_in = *in_.offset(i) as *const f64;
                        let channel_out = *out_.offset(i) as *mut f64;
                        for j in 0..num_samples as isize {
                            *channel_out.offset(j) = *channel_in.offset(j) * self.gain;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        kResultOk
    }

    unsafe fn get_tail_samples(&self) -> u32 {
        info!("Called: AGainProcessor::get_tail_samples()");

        0
    }
}

#[VST3(implements(IEditController, IUnitInfo))]
pub struct AGainController {
    units: Vec<UnitInfo>,
    parameters: Vec<(ParameterInfo, f64)>,
    context: *mut c_void,
    component_handler: *mut c_void,
}
impl AGainController {
    const CID: GUID = GUID {
        data: [
            0xD3, 0x9D, 0x5B, 0x65, 0xD7, 0xAF, 0x42, 0xFA, 0x84, 0x3F, 0x4A, 0xC8, 0x41, 0xEB,
            0x04, 0xF0,
        ],
    };
    pub fn new() -> Box<Self> {
        let units = vec![];
        let parameters = vec![];
        let context = null_mut();
        let component_handler = null_mut();
        AGainController::allocate(units, parameters, context, component_handler)
    }

    pub fn create_instance() -> *mut c_void {
        Box::into_raw(Self::new()) as *mut c_void
    }
}

impl IEditController for AGainController {
    unsafe fn set_component_state(&mut self, state: *mut c_void) -> tresult {
        info!("Called: AGainController::set_component_state()");

        if state.is_null() {
            return kResultFalse;
        }

        let state = state as *mut *mut _;
        let state: ComPtr<dyn IBStream> = ComPtr::new(state);

        let mut num_bytes_read = 0;
        let mut saved_gain = 0.0;
        let mut saved_bypass = false;
        let gain_ptr = &mut saved_gain as *mut f64 as *mut c_void;
        let bypass_ptr = &mut saved_bypass as *mut bool as *mut c_void;

        state.read(gain_ptr, mem::size_of::<f64>() as i32, &mut num_bytes_read);
        state.read(
            bypass_ptr,
            mem::size_of::<bool>() as i32,
            &mut num_bytes_read,
        );

        info!("saved_gain: {}", saved_gain);
        info!("saved_bypass: {}", saved_bypass);

        self.set_param_normalized(0, saved_gain);
        self.set_param_normalized(1, if saved_bypass { 1.0 } else { 0.0 });

        kResultOk
    }
    unsafe fn set_state(&mut self, _state: *mut c_void) -> tresult {
        info!("Called: AGainController::set_state()");

        kResultOk
    }
    unsafe fn get_state(&mut self, _state: *mut c_void) -> tresult {
        info!("Called: AGainController::get_state()");

        kResultOk
    }
    unsafe fn get_parameter_count(&self) -> i32 {
        info!("Called: AGainController::get_parameter_count()");

        self.parameters.len() as i32
    }
    unsafe fn get_parameter_info(&self, param_index: i32, info: *mut ParameterInfo) -> tresult {
        info!(
            "Called: AGainController::get_parameter_info(), args: param_index = {}",
            param_index
        );

        if param_index >= 0 && param_index < self.parameters.len() as i32 {
            *info = self.parameters[param_index as usize].0.clone();
            return kResultTrue;
        }
        kResultFalse
    }
    unsafe fn get_param_string_by_value(
        &self,
        id: u32,
        value_normalized: f64,
        string: *mut TChar,
    ) -> tresult {
        info!("Called: AGainProcessor::get_param_string_by_value()");
        match id {
            0 => {
                let value = format!("{:.0}", value_normalized * 100.0);
                wstrcpy(&value, string);
                kResultTrue
            }
            _ => kResultFalse,
        }
    }
    unsafe fn get_param_value_by_string(
        &self,
        _id: u32,
        _string: *mut TChar,
        _value_normalized: *mut f64,
    ) -> tresult {
        info!("Called: AGainController::get_param_value_by_string()");

        kResultFalse
    }
    unsafe fn normalized_param_to_plain(&self, id: u32, value_normalized: f64) -> f64 {
        info!("Called: AGainController::normalized_param_to_plain()");

        match id {
            0 => value_normalized * 100.0,
            1 => value_normalized,
            _ => unreachable!(),
        }
    }
    unsafe fn plain_param_to_normalized(&self, id: u32, plain_value: f64) -> f64 {
        info!("Called: AGainController::plain_param_normalized()");

        match id {
            0 => plain_value / 100.0,
            1 => plain_value,
            _ => unreachable!(),
        }
    }
    unsafe fn get_param_normalized(&self, id: u32) -> f64 {
        info!("Called: AGainController::get_param_normalized()");

        match id {
            0 => self.parameters[0].1,
            1 => self.parameters[1].1,
            _ => unreachable!(),
        }
    }
    unsafe fn set_param_normalized(&mut self, id: u32, value: f64) -> tresult {
        info!("Called: AGainController::set_param_normalized()");

        match id {
            0 => {
                self.parameters[0].1 = value;
                kResultTrue
            }
            1 => {
                self.parameters[1].1 = value;
                kResultTrue
            }
            _ => kResultFalse,
        }
    }
    unsafe fn set_component_handler(&mut self, handler: *mut c_void) -> tresult {
        info!("Called: AGainController::set_component_handler()");

        if self.component_handler == handler {
            return kResultTrue;
        }

        if !self.component_handler.is_null() {
            let component_handler = self.component_handler as *mut *mut _;
            let component_handler: ComPtr<dyn IComponentHandler> = ComPtr::new(component_handler);
            component_handler.release();
        }

        self.component_handler = handler;
        if !self.component_handler.is_null() {
            let component_handler = self.component_handler as *mut *mut _;
            let component_handler: ComPtr<dyn IComponentHandler> = ComPtr::new(component_handler);
            component_handler.add_ref();
        }

        kResultTrue
    }
    unsafe fn create_view(&self, _name: FIDString) -> *mut c_void {
        info!("Called: AGainController::create_view()");

        null_mut()
    }
}

impl IPluginBase for AGainController {
    unsafe fn initialize(&mut self, context: *mut c_void) -> tresult {
        info!("Called: AGainController::initialize()");

        if !self.context.is_null() {
            return kResultFalse;
        }
        self.context = context;

        let mut unit_info = UnitInfo {
            id: 1,
            parent_unit_id: 0,
            name: [0; 128],
            program_list_id: -1,
        };
        wstrcpy("Unit1", unit_info.name.as_mut_ptr());
        self.units.push(unit_info);

        let mut gain_parameter = ParameterInfo {
            id: 0,
            title: [0; 128],
            short_title: [0; 128],
            units: [0; 128],
            step_count: 0,
            default_normalized_value: 0.5,
            unit_id: 1,
            flags: kCanAutomate as i32,
        };
        wstrcpy("Gain", gain_parameter.title.as_mut_ptr());
        wstrcpy("Gain", gain_parameter.short_title.as_mut_ptr());
        wstrcpy("%", gain_parameter.units.as_mut_ptr());
        self.parameters.push((gain_parameter, 1.0));

        let mut bypass_parameter = ParameterInfo {
            id: 1,
            title: [0; 128],
            short_title: [0; 128],
            units: [0; 128],
            step_count: 1,
            default_normalized_value: 0.0,
            unit_id: 0,
            flags: kCanAutomate as i32 | kIsBypass as i32,
        };
        wstrcpy("Bypass", bypass_parameter.title.as_mut_ptr());
        self.parameters.push((bypass_parameter, 0.0));

        kResultOk
    }
    unsafe fn terminate(&mut self) -> tresult {
        info!("Called: AGainController::terminate()");

        self.units.clear();
        self.parameters.clear();

        if !self.component_handler.is_null() {
            let component_handler = self.component_handler as *mut *mut _;
            let component_handler: ComPtr<dyn IComponentHandler> = ComPtr::new(component_handler);
            component_handler.release();
            self.component_handler = null_mut();
        }

        self.context = null_mut();
        kResultOk
    }
}

impl IUnitInfo for AGainController {
    unsafe fn get_unit_count(&self) -> i32 {
        info!("Called: AGainController::get_unit_count()");

        1
    }

    unsafe fn get_unit_info(&self, unit_index: i32, info: *mut UnitInfo) -> i32 {
        info!("Called: AGainController::get_unit_info()");

        if unit_index >= 0 && unit_index < self.units.len() as i32 {
            *info = self.units[unit_index as usize].clone();
            return kResultTrue;
        }
        kResultFalse
    }

    unsafe fn get_program_list_count(&self) -> i32 {
        info!("Called: AGainController::get_program_list_count()");

        0
    }

    unsafe fn get_program_list_info(&self, _list_index: i32, _info: *mut ProgramListInfo) -> i32 {
        info!("Called: AGainController::get_program_list_info()");

        kResultFalse
    }

    unsafe fn get_program_name(
        &self,
        _list_id: i32,
        _program_index: i32,
        _name: [i16; 128],
    ) -> i32 {
        info!("Called: AGainController::get_program_name()");

        kResultFalse
    }

    unsafe fn get_program_info(
        &self,
        _list_id: i32,
        _program_index: i32,
        _attribute_id: *const i8,
        _attribute_value: [i16; 128],
    ) -> i32 {
        info!("Called: AGainController::get_program_info()");

        kResultFalse
    }

    unsafe fn has_program_pitch_names(&self, _id: i32, _index: i32) -> i32 {
        info!("Called: AGainController::has_program_pitch_names()");

        kResultFalse
    }

    unsafe fn get_program_pitch_name(
        &self,
        _id: i32,
        _index: i32,
        _pitch: i16,
        _name: *mut i16,
    ) -> i32 {
        info!("Called: AGainController::get_program_pitch_name()");

        kResultFalse
    }

    unsafe fn get_selected_unit(&self) -> i32 {
        info!("Called: AGainController::get_selected_unit()");

        0
    }

    unsafe fn select_unit(&self, _id: i32) -> i32 {
        info!("Called: AGainController::select_unit()");

        kResultFalse
    }

    unsafe fn get_unit_by_bus(
        &self,
        _type_: i32,
        _dir: i32,
        _index: i32,
        _channel: i32,
        _unit_id: *mut i32,
    ) -> i32 {
        info!("Called: AGainController::set_unit_by_bus()");

        kResultFalse
    }

    unsafe fn set_unit_program_data(
        &self,
        _list_or_unit: i32,
        _program_index: i32,
        _data: *mut dyn IBStream,
    ) -> i32 {
        info!("Called: AGainController::set_unit_program_data()");

        kResultFalse
    }
}

#[VST3(implements(IPluginFactory2, IPluginFactory))]
pub struct Factory {}
impl Factory {
    fn new() -> Box<Self> {
        Self::allocate()
    }

    pub fn create_instance() -> *mut c_void {
        Box::into_raw(Self::new()) as *mut c_void
    }
}

impl IPluginFactory2 for Factory {
    unsafe fn get_class_info2(&self, index: i32, info: *mut PClassInfo2) -> tresult {
        info!("Called: Factory::get_class_info2()");

        match index {
            0 => {
                let info = &mut *info;
                info.cid = AGainProcessor::CID;
                info.cardinality = ClassCardinality::kManyInstances as i32;
                strcpy("Audio Module Class", info.category.as_mut_ptr());
                strcpy("AGain VST3", info.name.as_mut_ptr());
                info.class_flags = 1;
                strcpy("Fx", info.subcategories.as_mut_ptr());
                strcpy("0.1.0", info.version.as_mut_ptr());
                strcpy("VST 3.6.14", info.sdk_version.as_mut_ptr());
            }
            1 => {
                let info = &mut *info;
                info.cid = AGainController::CID;
                info.cardinality = ClassCardinality::kManyInstances as i32;
                strcpy("Component Controller Class", info.category.as_mut_ptr());
                strcpy("AGain VST3 Controller", info.name.as_mut_ptr());
                info.class_flags = 0;
                strcpy("", info.subcategories.as_mut_ptr());
                strcpy("0.1.0", info.version.as_mut_ptr());
                strcpy("VST 3.6.14", info.sdk_version.as_mut_ptr());
            }
            _ => {
                info!("Invalid class info ID {}", index);
                return kInvalidArgument;
            }
        }

        kResultOk
    }
}

impl IPluginFactory for Factory {
    unsafe fn get_factory_info(&self, info: *mut PFactoryInfo) -> tresult {
        info!("Called: Factory::get_factory_info()");

        let info = &mut *info;
        strcpy("rust.audio", info.vendor.as_mut_ptr());
        strcpy("https://rust.audio", info.url.as_mut_ptr());
        strcpy("mailto://mrkcvzz@gmail.com", info.email.as_mut_ptr());
        info.flags = 8;

        kResultOk
    }
    unsafe fn count_classes(&self) -> i32 {
        info!("Called: Factory::count_classes()");
        2
    }
    unsafe fn get_class_info(&self, index: i32, info: *mut PClassInfo) -> tresult {
        info!("Called: Factory::get_class_info()");

        match index {
            0 => {
                let info = &mut *info;
                info.cardinality = ClassCardinality::kManyInstances as i32;
                info.cid = AGainProcessor::CID;
                strcpy("Audio Module Class", info.category.as_mut_ptr());
                strcpy("AGain VST3", info.name.as_mut_ptr());
            }
            1 => {
                let info = &mut *info;
                info.cardinality = ClassCardinality::kManyInstances as i32;
                info.cid = AGainController::CID;
                strcpy("Component Controller Class", info.category.as_mut_ptr());
                strcpy("AGain VST3 Controller", info.name.as_mut_ptr());
            }
            _ => {
                info!("Invalid class info ID {}", index);
                return kInvalidArgument;
            }
        }
        kResultOk
    }
    unsafe fn create_instance(
        &self,
        cid: *mut IID,
        _iid: *mut IID,
        obj: *mut *mut c_void,
    ) -> tresult {
        let processor_cid = AGainProcessor::CID;
        let controller_cid = AGainController::CID;

        info!("Creating instance of {:?}", *cid);
        if (*cid) == processor_cid {
            *obj = AGainProcessor::create_instance();
            info!("Created instance of AGainProcessor");
            return kResultOk;
        }
        if (*cid) == controller_cid {
            *obj = AGainController::create_instance();
            info!("Created instance of AGainController");
            return kResultOk;
        }
        warn!("CID not found");
        kResultFalse
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn InitDll() -> bool {
    true
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn ExitDll() -> bool {
    true
}

#[cfg(target_os = "linux")]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn ModuleEntry(_: *mut c_void) -> bool {
    true
}

#[cfg(target_os = "linux")]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn ModuleExit() -> bool {
    info!("Module exited");
    true
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn bundleEntry(_: *mut c_void) -> bool {
    true
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn bundleExit() -> bool {
    info!("Module exited");
    true
}

static mut INIT_LOGGER: bool = false;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn GetPluginFactory() -> *mut c_void {
    if !INIT_LOGGER {
        let log_path = std::env::var("VST3_LOG_PATH");
        match log_path {
            Ok(path) => {
                match Logger::with_env_or_str("info")
                    .log_to_file()
                    .directory(path)
                    .format(opt_format)
                    .start()
                {
                    Ok(_) => info!("Started logger..."),
                    Err(_) => (),
                }
            }
            Err(_) => (),
        }
        INIT_LOGGER = true;
    }
    Factory::create_instance()
}
