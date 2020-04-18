#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(missing_docs)]

#[cfg(windows)]
pub use winapi::{
    ctypes::{c_ulong, c_void},
    shared::{
        guiddef::{IsEqualGUID, GUID, IID, REFCLSID, REFIID},
        minwindef::{BOOL, DWORD, HKEY, LPVOID},
        ntdef::HRESULT,
        winerror::{ERROR_SUCCESS, E_NOINTERFACE, E_POINTER, FAILED, S_FALSE, S_OK},
        wtypesbase::CLSCTX_INPROC_SERVER,
    },
};

#[cfg(not(windows))]
pub mod xplatform {
    pub use std::os::raw::{c_int, c_long, c_ulong, c_void};
    
    #[repr(align(16))]
    #[derive(Copy, Clone)]
    pub struct GUID {
        #[allow(missing_docs)]
        pub data1: u32,
        #[allow(missing_docs)]
        pub data2: u16,
        #[allow(missing_docs)]
        pub data3: u16,
        #[allow(missing_docs)]
        pub data4: [u8; 8],
    }

    pub type IID = GUID;
    pub type REFIID = *const IID;
    pub type REFCLSID = *const IID;
    pub type BOOL = c_int;
    pub type DWORD = c_ulong;
    pub type LPVOID = *mut c_void;
    pub type HKEY = *mut c_void;
    pub type HRESULT = c_long;

    #[inline]
    pub fn FAILED(hr: HRESULT) -> bool {
        hr < 0
    }
    pub const ERROR_SUCCESS: DWORD = 0;
    pub const S_OK: HRESULT = 0;
    pub const S_FALSE: HRESULT = 1;
    pub const E_NOINTERFACE: HRESULT = 0x80004002;
    pub const E_POINTER: HRESULT = 0x80004003;
    pub const CLSCTX_INPROC_SERVER: c_int = 0x1;

    #[inline]
    pub fn IsEqualGUID(g1: &GUID, g2: &GUID) -> bool {
        let a = unsafe { &*(g1 as *const _ as *const [u32; 4]) };
        let b = unsafe { &*(g2 as *const _ as *const [u32; 4]) };
        a[0] == b[0] && a[1] == b[1] && a[2] == b[2] && a[3] == b[3]
    }
}

#[cfg(not(windows))]
pub use xplatform::*;
