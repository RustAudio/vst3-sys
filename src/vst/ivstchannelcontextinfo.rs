use vst3_com::interfaces::IUnknown;
use vst3_com::{c_void, com_interface};

use crate::base::tresult;
use crate::vst::{CString, ColorSpec};

#[com_interface("0F194781-8D98-4ADA-BBA0-C1EFC011D8D0")]
pub trait IInfoListener: IUnknown {
    unsafe fn set_channel_context_infos(&mut self, list: *mut c_void) -> tresult;
}

pub enum ChannelPluginLocation {
    kPreVolumeFader = 0,
    kPostVolumeFader = 1,
    kUsedAsPanner = 2,
}

pub type ColorComponent = u8;

pub fn get_blue(cs: ColorSpec) -> ColorComponent {
    (cs & 0x0000_00FF) as ColorComponent
}

pub fn get_green(cs: ColorSpec) -> ColorComponent {
    ((cs >> 8) & 0x0000_00FF) as ColorComponent
}

pub fn get_red(cs: ColorSpec) -> ColorComponent {
    ((cs >> 16) & 0x0000_00FF) as ColorComponent
}

pub fn get_alpha(cs: ColorSpec) -> ColorComponent {
    ((cs >> 24) & 0x0000_00FF) as ColorComponent
}

pub const kChannelUIDKey: CString = b"channel uid".as_ptr() as *const _;
pub const kChannelUIDLengthKey: CString = b"channel uid length".as_ptr() as *const _;
pub const kChannelNameKey: CString = b"channel name".as_ptr() as *const _;
pub const kChannelNameLengthKey: CString = b"channel name length".as_ptr() as *const _;
pub const kChannelColorKey: CString = b"channel color".as_ptr() as *const _;
pub const kChannelIndexKey: CString = b"channel index".as_ptr() as *const _;
pub const kChannelIndexNamespaceOrderKey: CString =
    b"channel index namespace order".as_ptr() as *const _;
pub const kChannelIndexNamespaceKey: CString = b"channel index namespace".as_ptr() as *const _;
pub const kChannelIndexNamespaceLengthKey: CString =
    b"channel index namespace length".as_ptr() as *const _;
pub const kChannelImageKey: CString = b"channel image".as_ptr() as *const _;
pub const kChannelPluginLocationKey: CString = b"channel plugin location".as_ptr() as *const _;
