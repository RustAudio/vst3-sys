use vst3_com::interfaces::IUnknown;
use vst3_com::{c_void, com_interface};

use crate::base::tresult;
use crate::vst::{ParamID, String128};

type UCoord = i32;

#[com_interface("69F11617-D26B-400D-A4B6-B9647B6EBBAB")]
pub trait IComponentHandler3: IUnknown {
    unsafe fn create_context_menu(&self, plug_view: *mut c_void, param_id: ParamID) -> *mut c_void;
}

#[com_interface("3CDF2E75-85D3-4144-BF86-D36BD7C4894D")]
pub trait IContextMenuTarget: IUnknown {
    unsafe fn execute_menu_item(&self, tag: i32) -> tresult;
}

#[repr(C)]
pub struct IContextMenuItem {
    pub name: String128,
    pub tag: i32,
    pub flags: i32,
}

pub enum Flags {
    kIsSeparator = 1,
    kIsDisabled = 2,
    kIsChecked = 4,
    kIsGroupStart = 10,
    kIsGroupEnd = 17,
}

#[com_interface("2E93C863-0C9C-4588-97DB-ECF5AD17817D")]
pub trait IContextMenu: IUnknown {
    unsafe fn get_item_count(&self) -> i32;
    unsafe fn get_item(
        &self,
        index: i32,
        item: *mut IContextMenuItem,
        target: *mut *mut c_void,
    ) -> tresult;
    unsafe fn add_item(&self, item: *mut IContextMenuItem, target: *mut c_void) -> tresult;
    unsafe fn remove_item(&self, item: *mut IContextMenuItem, target: *mut c_void) -> tresult;
    unsafe fn popup(&self, x: UCoord, y: UCoord) -> tresult;
}
