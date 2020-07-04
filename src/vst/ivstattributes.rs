use vst3_com::{com_interface, c_void};
use vst3_com::interfaces::iunknown::IUnknown;

use crate::base::{char8, tchar, tresult};

pub type AttrID = *const char8;

// todo: contextualize documentation by referencing elements of the crate in the proper way

/// Attribute list used in IMessage and IStreamAttributes.
///
/// An attribute list associates values with a key (id: some predefined keys could be found
/// in \ref presetAttributes).
#[com_interface("1E5F0AEB-CC7F-4533-A254-401138AD5EE4")]
pub trait IAttributeList: IUnknown {
    /// Sets integer value.
    unsafe fn set_int(&self, id: AttrID, value: i64) -> tresult;
    /// Gets integer value.
    unsafe fn get_int(&self, id: AttrID, value: *mut i64) -> tresult;
    /// Sets float value.
    unsafe fn set_float(&self, id: AttrID, value: f64) -> tresult;
    /// Gets float value.
    unsafe fn get_float(&self, id: AttrID, value: *mut f64) -> tresult;
    /// Sets string value (UTF-16).
    unsafe fn set_string(&self, id: AttrID, value: *const tchar, size: u32) -> tresult;
    /// Gets string value (UTF-16). Note that size is in bytes and doesn't represent the string length.
    unsafe fn get_string(&self, id: AttrID, value: *mut tchar, size: u32) -> tresult;
    /// Sets binary data.
    unsafe fn set_binary(&self, id: AttrID, ptr: *const c_void, size: u32) -> tresult;
    /// Gets binary data.
    unsafe fn get_binary(&self, id: AttrID, ptr: *const *mut c_void, size: *mut u32) -> tresult;
}

/// Interface to access preset meta information from stream, used for example in set_state in order
/// to inform the plug-in about the current context in which this preset loading occurs (Project
/// context or Preset load (see \ref StateType)) or used to get the full file path of the loaded preset
/// (if available).
#[com_interface("D6CE2FFC-EFAF-4B8C-9E74-F1BB12DA44B4")]
pub trait IStreamAttributes: IUnknown {
    /// Gets filename (without file extension) of the stream.
    unsafe fn get_filename(&self, name: *mut tchar) -> tresult;
    /// Gets meta information list. (Returns a pointer to IAttributeList)
    unsafe fn get_attributes(&self) -> *mut c_void;
}
