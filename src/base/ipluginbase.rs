use crate::base::{char16, char8, tresult};
use bitflags::bitflags;
use vst3_com::interfaces::iunknown::IUnknown;
use vst3_com::{c_void, com_interface, IID};

bitflags! {
    pub struct FactoryFlags: i32 {
        const kNoFlags = 0;
        const kClassesDiscardable = 1;
        const kLicenseCheck = 2;
        const kComponentNonDiscardable = 4;
        const kUnicode = 8;
    }
}

#[repr(C)]
pub struct PFactoryInfo {
    pub vendor: [char8; 64],
    pub url: [char8; 256],
    pub email: [char8; 128],
    pub flags: i32,
}

pub enum ClassCardinality {
    kManyInstances = 0x7FFF_FFFF,
}

#[repr(C)]
pub struct PClassInfo {
    pub cid: IID,
    pub cardinality: i32,
    pub category: [char8; 32],
    pub name: [char8; 64],
}

#[repr(C)]
pub struct PClassInfo2 {
    pub cid: IID,
    pub cardinality: i32,
    pub category: [char8; 32],
    pub name: [char8; 64],
    pub class_flags: u32,
    pub subcategories: [char8; 128],
    pub vendor: [char8; 64],
    pub version: [char8; 64],
    pub sdk_version: [char8; 64],
}

#[repr(C)]
pub struct PClassInfoW {
    pub cid: IID,
    pub cardinality: i32,
    pub category: [char8; 32],
    pub name: [char16; 64],
    pub class_flags: u32,
    pub subcategories: [char8; 128],
    pub vendor: [char16; 64],
    pub version: [char16; 64],
    pub sdk_version: [char16; 64],
}

/// Basic interface to a Plug-in component.
#[com_interface("22888DDB-156E-45AE-8358-B34808190625")]
pub trait IPluginBase: IUnknown {
    unsafe fn initialize(&mut self, context: *mut c_void) -> tresult;
    unsafe fn terminate(&mut self) -> tresult;
}

/// Class factory that any Plug-in defines for creating class instances.
#[com_interface("7A4D811C-5211-4A1F-AED9-D2EE0B43BF9F")]
pub trait IPluginFactory: IUnknown {
    unsafe fn get_factory_info(&self, info: *mut PFactoryInfo) -> tresult;
    unsafe fn count_classes(&self) -> i32;
    unsafe fn get_class_info(&self, index: i32, info: *mut PClassInfo) -> tresult;
    unsafe fn create_instance(
        &self,
        cid: *mut IID,
        _iid: *mut IID,
        obj: *mut *mut c_void,
    ) -> tresult;
}

#[com_interface("0007B650-F24B-4C0B-A464-EDB9F00B2ABB")]
pub trait IPluginFactory2: IPluginFactory {
    unsafe fn get_class_info2(&self, index: i32, info: *mut PClassInfo2) -> tresult;
}

#[com_interface("4555A2AB-C123-4E57-9B12-291036878931")]
pub trait IPluginFactory3: IPluginFactory2 {
    unsafe fn get_class_info_unicode(&self, idx: i32, info: *mut PClassInfoW) -> tresult;
    unsafe fn set_host_context(&self, context: *mut dyn IUnknown);
}
