use crate::base::{tchar, tresult, TBool};
use crate::vst::IMessage;
use vst3_com::interfaces::iunknown::IUnknown;
use vst3_com::{com_interface, IID};

#[com_interface("E564A636-6A8B-8CAF-DB2D-496958E595CC")]
pub trait IHostApplication: IUnknown {
    unsafe fn get_name(&self, name: tchar) -> tresult;
    unsafe fn create_instance(
        &self,
        cid: *mut IID,
        _iid: *mut IID,
        obj: *mut *mut c_void,
    ) -> tresult;
}

#[inline]
pub unsafe fn allocate_message(host: *mut dyn IHostApplication) -> Option<*mut dyn IMessage> {
    // TODO: Impl the FUID class first for Interface::iid.to_tuid to work
    let mut iid: IID = Default::default;
    // IMessage::iid.to_tuid(iid);
    let m: *mut IMessage = std::ptr::null_mut();
    if (host.createInstance(iid, iid, m as *mut *mut c_void /* or invoke as_mut_ptr()? */) == kResultOk) {
        Some(m)
    }
    None
}

#[com_interface("61AC6ED3-85BB-7BB9-1D1C-47E229633AEC")]
pub trait IVst3ToVst2Wrapper: IUnknown { }

#[com_interface("44AA97B6-91B0-0B6F-C095-4688A3B8C6C5")]
pub trait IVst3ToAUWrapper: IUnknown { }

#[com_interface("C6F4BE93-2CB3-1B95-60C5-62426D319DC6")]
pub trait IVst3ToAAXWrapper: IUnknown { }

#[com_interface("E39F35F7-0088-50B7-42CF-4BF944149067")]
pub trait IVst3WrapperMPESupport: IUnknown {
    unsafe fn enable_mpe_input_processing(&self, state: TBool) -> tresult;
    unsafe fn set_mpe_input_device_settings(
        &self,
        master_channel: i32,
        member_begin_channel: i32,
        member_end_channel: i32
        ) -> tresult;
}
