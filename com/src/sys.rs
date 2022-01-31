//! Types for interacting with COM related system APIs
//!
//! Stripped down to only the parts that overlap with VST3. `vst3_sys::base::funknown` contains the
//! correct constants to use for interacting with VST3 APIs.
use std::cmp::PartialEq;

/// A Windows result code
pub type HRESULT = i32;

/// Equivalent of the [FAILED macro](https://docs.microsoft.com/en-us/windows/win32/api/winerror/nf-winerror-failed)
#[cfg(not(target_os = "windows"))]
#[allow(non_snake_case, clippy::manual_range_contains)]
pub fn FAILED(result: HRESULT) -> bool {
    // See `vst3_sys::base::funknown` for all return codes
    result < 0 || result >= E_INVALIDARG
}
/// Equivalent of the [FAILED macro](https://docs.microsoft.com/en-us/windows/win32/api/winerror/nf-winerror-failed)
#[cfg(target_os = "windows")]
#[allow(non_snake_case)]
pub fn FAILED(result: HRESULT) -> bool {
    result < 0
}

/// BOOL type
pub type BOOL = i32;

/// No error
pub const S_OK: HRESULT = 0;
/// No error
pub const NOERROR: HRESULT = 0;
/// False
pub const S_FALSE: HRESULT = 1;

/// Argument was invalid
#[cfg(not(target_os = "windows"))]
pub const E_INVALIDARG: HRESULT = 2;
/// Argument was invalid
#[cfg(target_os = "windows")]
pub const E_INVALIDARG: HRESULT = -0x7FF8_FFA9;

/// No interface found
#[cfg(not(target_os = "windows"))]
pub const E_NOINTERFACE: HRESULT = -1;
/// No interface found
#[cfg(target_os = "windows")]
pub const E_NOINTERFACE: HRESULT = -0x7FFF_BFFE;

/// A globally unique identifier
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct GUID {
    /// bytes of the GUID
    pub data: [u8; 16],
}

/// An interface ID
pub type IID = GUID;

/// A class ID
pub type CLSID = GUID;

impl std::fmt::Debug for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|n| format!("{:02X?}", n))
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}
