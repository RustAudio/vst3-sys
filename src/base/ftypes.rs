use std::os::raw::c_char;

pub type TSize = i64;
pub type tresult = i32;
pub type TBool = u8;
pub type char8 = c_char;
pub type tchar = i16;
pub type CStringA = *const char8;
pub type CStringW = *const tchar;
pub type FIDString = *const char8;
