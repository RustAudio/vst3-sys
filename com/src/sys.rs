//! Types for interacting with COM related system APIs
use std::cmp::PartialEq;
use std::ffi::c_void;

/// A Windows result code
pub type HRESULT = i32;

/// Equivalent of the [FAILED macro](https://docs.microsoft.com/en-us/windows/win32/api/winerror/nf-winerror-failed)
#[allow(non_snake_case)]
pub fn FAILED(result: HRESULT) -> bool {
    result < 0
}

/// BOOL type
pub type BOOL = i32;
/// LSTATUS type
pub type LSTATUS = i32;
/// HKEY type
pub type HKEY = *mut c_void;

/// No error
pub const S_OK: HRESULT = 0;
/// No error
pub const NOERROR: HRESULT = 0;
/// False
pub const S_FALSE: HRESULT = 1;

/// Argument was invalid
pub const E_INVALIDARG: HRESULT = -0x7FF8_FFA9;
/// No interface found
pub const E_NOINTERFACE: HRESULT = -0x7FFF_BFFE;
/// Invalid pointer
pub const E_POINTER: HRESULT = -0x7FFF_BFFD;

/// No aggregation for CoClass
pub const CLASS_E_NOAGGREGATION: HRESULT = -0x7FFB_FEF0;
/// Class is not available
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = -0x7FFB_FEEF;

/// No error
pub const ERROR_SUCCESS: u32 = 0;
/// Registration error
pub const SELFREG_E_CLASS: HRESULT = -0x7FFB_FDFF;
/// A in process server
pub const CLSCTX_INPROC_SERVER: u32 = 0x1;

/// An single threaded apartment (STA)
pub const COINIT_APARTMENTTHREADED: u32 = 0x2;
/// An multi threaded apartment (STA)
pub const COINIT_MULTITHREADED: u32 = 0x0;

/// A globally unique identifier
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
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

#[cfg(windows)]
#[link(name = "ole32")]
extern "system" {
    /// Keeps MTA support active when no MTA threads are running
    pub fn CoIncrementMTAUsage(cookie: *mut c_void) -> HRESULT;
    /// Creates the specified registry key
    pub fn RegCreateKeyExA(
        hKey: HKEY,
        lpSubKey: *const i8,
        Reserved: u32,
        lpClass: *mut u8,
        dwOptions: u32,
        samDesired: u32,
        lpSecurityAttributes: *mut c_void,
        phkResult: *mut HKEY,
        lpdwDisposition: *mut u32,
    ) -> LSTATUS;
    /// Retrieves the fully qualified path for the file that contains the specified module
    pub fn GetModuleFileNameA(hModule: *mut c_void, lpFilename: *mut i8, nSize: u32) -> u32;
    /// Closes a handle to the specified registry key
    pub fn RegCloseKey(hKey: HKEY) -> LSTATUS;
    /// Sets the data and type of a specified value under a registry key
    pub fn RegSetValueExA(
        hKey: HKEY,
        lpValueName: *const i8,
        Reserved: u32,
        dwType: u32,
        lpData: *const u8,
        cbData: u32,
    ) -> LSTATUS;
    /// Deletes a subkey and its values
    pub fn RegDeleteKeyA(hKey: HKEY, lpSubKey: *const i8) -> LSTATUS;
    /// Retrieves a module handle for the specified module
    pub fn GetModuleHandleA(lpModuleName: *const i8) -> *mut c_void;
    /// Initializes the COM library for use by the calling thread, sets the thread's concurrency model, and creates a new apartment for the thread if one is required
    pub fn CoInitializeEx(pvReserved: *mut c_void, dwCoInit: u32) -> HRESULT;
    /// Provides a pointer to an interface on a class object associated with a specified CLSID
    pub fn CoGetClassObject(
        rclsid: *const IID,
        dwClsContext: u32,
        pvReserved: *mut c_void,
        riid: *const IID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    /// Creates and default-initializes a single object of the class associated with a specified CLSID
    pub fn CoCreateInstance(
        rclsid: *const IID,
        pUnkOuter: *mut c_void,
        dwClsContext: u32,
        riid: *const IID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    /// Closes the COM library on the current thread, unloads all DLLs loaded by the thread, frees any other resources that the thread maintains, and forces all RPC connections on the thread to close
    pub fn CoUninitialize();
}
