use crate::base::{char16, char8, tresult};
use bitflags::bitflags;
use com::interfaces::iunknown::IUnknown;
use com::{c_void, com_interface, IID};

bitflags! {
    pub struct FactoryFlags: i32 {
        const kNoFlags = 0;
        const kClassesDiscardable = 1;
        const kLicenseCheck = 2;
        const kComponentNonDiscardable = 4;
        const kUnicode = 8;
    }
}

#[repr(align(16))]
pub struct PFactoryInfo {
    vendor: [char8; 64],
    url: [char8; 256],
    email: [char8; 128],
    flags: i32,
}

#[repr(align(16))]
pub struct PClassInfo {
    cid: IID,
    cardinality: i32,
    category: [char8; 32],
    name: [char8; 64],
}

#[repr(align(16))]
pub struct PClassInfo2 {
    cid: IID,
    cardinality: i32,
    category: [char8; 32],
    name: [char8; 64],
    class_flags: u32,
    subcategories: [char8; 128],
    vendor: [char8; 64],
    version: [char8; 64],
    sdk_version: [char8; 64],
}

#[repr(align(16))]
pub struct PClassInfoW {
    cid: IID,
    cardinality: i32,
    category: [char8; 32],
    name: [char16; 64],
    class_flags: u32,
    subcategories: [char8; 128],
    vendor: [char16; 64],
    version: [char16; 64],
    sdk_version: [char16; 64],
}
/// Renamed from IPLuginBase
#[com_interface("22888DDB-156E-45AE-8358-B34808190625")]
pub trait IPlugin: IUnknown {
    unsafe fn initialize(&self, context: *mut dyn IUnknown) -> tresult;
    unsafe fn terminate(&self) -> tresult;
}

#[com_interface("7A4D811C-5211-4A1F-AED9-D2EE0B43BF9F")]
pub trait IPluginFactory: IUnknown {
    unsafe fn get_factory_info(&self, info: *mut PFactoryInfo) -> tresult;
    unsafe fn count_classes(&self) -> i32;
    unsafe fn get_class_info(&self, idx: i32, info: *mut PClassInfo) -> tresult;
    unsafe fn create_instance(
        &self,
        cid: *mut IID,
        _iid: *mut IID,
        obj: *mut *mut c_void,
    ) -> tresult;
}

#[com_interface("0007B650-F24B-4C0B-A464-EDB9F00B2ABB")]
pub trait IPluginFactory2: IPluginFactory {
    unsafe fn get_factory_info2(&self, info: *mut PClassInfo2) -> tresult;
}

#[com_interface("4555A2AB-C123-4E57-9B12-291036878931")]
pub trait IPluginFactory3: IPluginFactory2 {
    unsafe fn get_class_info_unicode(&self, idx: i32, info: *mut PClassInfoW) -> tresult;
    unsafe fn set_host_context(&self, context: *mut dyn IUnknown);
}
