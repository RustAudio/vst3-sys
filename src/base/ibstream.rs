use crate::base::tresult;
use com::{c_void, com_interface, interfaces::iunknown::IUnknown, REFIID};

const kIBSeekSet: i32 = 0;
const kIBSeekCur: i32 = 1;
const kIBSeekEnd: i32 = 2;

#[com_interface(C3BF6EA2-3099-4752-9B6B-F9901EE33E9B)]
pub trait IBStream: IUnknown {
    unsafe fn read(&self, buffer: *mut c_void, num_bytes: i32, num_bytes_read: *mut i32)
        -> tresult;
    unsafe fn write(
        &self,
        buffer: *mut c_void,
        num_bytes: i32,
        num_bytes_written: *mut i32,
    ) -> tresult;
    unsafe fn seek(&self, pos: i64, mode: i32, result: *mut i64) -> tresult;
    unsafe fn tell(&self, pos: *mut i64) -> tresult;
}
