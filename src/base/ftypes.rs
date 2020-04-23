/// Common types
use std::os::raw::c_char;
/// A size type
pub type TSize = i64;
/// Integer error code
pub type tresult = i32;
/// Boolean type
pub type TBool = u8;
/// A byte
pub type char8 = c_char;
/// A UTF-16 character
pub type tchar = i16;
/// A UTF-16 character
pub type char16 = tchar;
/// A null terminated C-String
pub type CStringA = *const char8;
/// A UTF16 string (wide string type)
pub type CStringW = *const tchar;
/// A string used for an FUnknown Identifier
pub type FIDString = *const char8;
