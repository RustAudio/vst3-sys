#![allow(unused_unsafe)]
use lazy_static::lazy_static;
use log::*;
use std::os::raw::{c_char, c_void};
use std::ptr::copy_nonoverlapping as memcpy;
use std::sync::Mutex;
use vst3_com::sys::GUID;
use vst3_com::IID;
use vst3_sys::base::{
    kInvalidArgument, kResultOk, tresult, IPluginBase, IPluginFactory, IUnknown, TBool,
};
use vst3_sys::vst::{
    BusDirection, BusDirections, BusFlags, BusInfo, IAudioPresentationLatency, IAudioProcessor,
    IAutomationState, IComponent, MediaTypes, ProcessData, ProcessSetup, RoutingInfo,
};
use vst3_sys::VST3;

#[VST3(implements(
    IAudioProcessor,
    IAudioPresentationLatency,
    IAutomationState,
    IPluginBase
))]
pub struct PassthruPlugin {}
pub struct PassthruController {}
impl PassthruPlugin {
    const CID: GUID = GUID {
        data1: 0x93684f1a,
        data2: 0x4611,
        data3: 0x9101,
        data4: [0x0, 0, 0xb4, 0x39, 0xe5, 0x64, 0x8a, 0xda],
    };
    pub fn new() -> Box<Self> {
        PassthruPlugin::allocate()
    }
}
#[VST3(implements(IPluginFactory))]
pub struct Factory {}

impl IAudioProcessor for PassthruPlugin {
    unsafe fn set_bus_arrangements(
        &self,
        _inputs: *mut u64,
        _num_ins: i32,
        _outputs: *mut u64,
        _num_outs: i32,
    ) -> i32 {
        unimplemented!()
    }

    unsafe fn get_bus_arrangements(&self, _dir: i32, _index: i32, _arr: *mut u64) -> i32 {
        unimplemented!()
    }

    unsafe fn can_process_sample_size(&self, _symbolic_sample_size: i32) -> i32 {
        unimplemented!()
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
    unsafe fn process(&mut self, _data: *mut ProcessData) -> tresult {
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
            let info = unsafe { &mut *info };
            if dir == BusDirections::kInput as i32 {
                info.direction = dir;
                info.bus_type = MediaTypes::kAudio as i32;
                info.channel_count = 2;
                info.flags = BusFlags::kDefaultActive.bits();
            } else {
                info.direction = dir;
                info.bus_type = MediaTypes::kAudio as i32;
                info.channel_count = 2;
                info.flags = BusFlags::kDefaultActive.bits();
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
        unimplemented!()
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

//todo: IComponent
//todo: IContextMenuTarget
//todo: IEditController
//todo: IEditController2
//todo: IMidiMapping
//todo: IEditControllerHostEditing
//todo: IInterAppAudioConnectionNotification
//todo: IInterAppAudioPresetManager
//todo: IConnectionPoint
//todo: IMidiLearn
//todo: INoteExpressionController
//todo: IKeyswitchController
//todo: INoteExpressionPhysicalUIMapping
//todo: IPrefetchableSupport
//todo: IXmlRepresentationController
//todo: IUnitInfo
//todo: IProgramListData
//todo: IUnitData
//todo: IPlugView
//todo: IPlugViewContentScaleSupport

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Factory implementation

impl Factory {
    fn new() -> Box<Self> {
        info!("instantiating factory...");
        Self::allocate()
    }
}

unsafe fn strcpy(src: &str, dst: *mut c_char) {
    memcpy(src.as_ptr() as *const c_void as *const _, dst, src.len());
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
                info.cardinality = 0x7FFFFFFF;
                info.cid = PassthruPlugin::CID.to_be();
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
        riid: *mut vst3_com::sys::GUID,
        ppv: *mut *mut core::ffi::c_void,
    ) -> i32 {
        //todo: figure out why this method fails in the validator
        let cid = *&*cid;
        let cmp = PassthruPlugin::CID;

        info!("creating instance of {:?}", cid);
        if cid.to_le() == cmp {
            let instance = PassthruPlugin::new();
            instance.add_ref();
            let hr = instance.query_interface(riid, ppv);
            instance.release();
            core::mem::forget(instance);
            return hr;
        } else {
            warn!("CID not found");
        }
        kResultOk
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// entry point and wrapping code to satisfy the borrow checker
// todo: cleanup singleton instance so this is less hacky

struct FactoryWrapper {
    factory: Box<Factory>,
}
unsafe impl Send for FactoryWrapper {}
unsafe impl Sync for FactoryWrapper {}
lazy_static! {
    static ref WRAPPER: Mutex<FactoryWrapper> = Mutex::new(FactoryWrapper {
        factory: Factory::new()
    });
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn GetPluginFactory() -> *mut c_void {
    info!("calling plugin factory");
    let factory = &mut *WRAPPER.lock().unwrap().factory;
    factory.add_ref();
    factory as *mut _ as *mut _
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn ModuleEntry(_: *mut c_void) -> bool {
    if let Err(e) = simple_logger::init() {
        println!("{:?}", e);
    }
    info!("Module entered");
    true
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn ModuleExit() -> bool {
    info!("Module exited");
    true
}
