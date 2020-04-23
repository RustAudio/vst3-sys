use crate::base::{tresult, FactoryFlags, IPluginBase, TBool};
use crate::vst::{BusDirection, IoMode, MediaType, String128};
use bitflags::bitflags;
use vst3_com::{c_void, com_interface, IID};

pub const kDefaultFactoryFlags: i32 = FactoryFlags::kUnicode.bits();

pub enum MediaTypes {
    kAudio = 0,
    kEvent = 1,
    kNumMediaTypes = 2,
}

pub enum BusDirections {
    kInput = 0,
    kOutput = 1,
}

pub enum BusTypes {
    kMain = 0,
    kAux = 1,
}

pub enum IoModes {
    kSimple = 0,
    kAdvanced = 1,
    kOfflineProcessing = 2,
}

bitflags! {
    pub struct BusFlags: u32 {
        const kDefaultActive = 1;
    }
}

#[repr(C)]
pub struct BusInfo {
    pub media_type: i32,
    pub direction: i32,
    pub channel_count: i32,
    pub name: String128,
    pub bus_type: i32,
    pub flags: u32,
}

#[repr(C)]
pub struct RoutingInfo {
    pub media_type: i32,
    pub bus_index: i32,
    pub channel: i32,
}

/// The `IComponent` interface is used by the plugin to determine
/// the input/output configurations for audio, midi, and event channels,
/// alert the plugin about which IO mode is currently active, activate
/// different busses, and get/set the state of t
#[com_interface("E831FF31-F2D5-4301-928E-BBEE25697802")]
pub trait IComponent: IPluginBase {
    unsafe fn get_controller_class_id(&self, tuid: *mut IID) -> tresult;
    unsafe fn set_io_mode(&self, mode: IoMode) -> tresult;
    unsafe fn get_bus_count(&self, type_: MediaType, dir: BusDirection) -> i32;
    unsafe fn get_bus_info(
        &self,
        type_: MediaType,
        dir: BusDirection,
        index: i32,
        info: *mut BusInfo,
    ) -> tresult;
    unsafe fn get_routing_info(
        &self,
        in_info: *mut RoutingInfo,
        out_info: *mut RoutingInfo,
    ) -> tresult;
    unsafe fn activate_bus(
        &mut self,
        type_: MediaType,
        dir: BusDirection,
        index: i32,
        state: TBool,
    ) -> tresult;
    unsafe fn set_active(&self, state: TBool) -> tresult;
    unsafe fn set_state(&mut self, state: *mut c_void) -> tresult;
    unsafe fn get_state(&mut self, state: *mut c_void) -> tresult;
}
