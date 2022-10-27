use crate::base::{char16, tresult, FIDString, TBool};
use crate::utils::SharedVstPtr;
use vst3_com::interfaces::iunknown::IUnknown;
use vst3_com::{c_void, com_interface};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ViewRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[com_interface("5BC32507-D060-49EA-A615-1B522B755B29")]
pub trait IPlugView: IUnknown {
    unsafe fn is_platform_type_supported(&self, type_: FIDString) -> tresult;
    unsafe fn attached(&self, parent: *mut c_void, type_: FIDString) -> tresult;
    unsafe fn removed(&self) -> tresult;
    unsafe fn on_wheel(&self, distance: f32) -> tresult;
    unsafe fn on_key_down(&self, key: char16, key_code: i16, modifiers: i16) -> tresult;
    unsafe fn on_key_up(&self, key: char16, key_code: i16, modifiers: i16) -> tresult;
    unsafe fn get_size(&self, size: *mut ViewRect) -> tresult;
    unsafe fn on_size(&self, new_size: *mut ViewRect) -> tresult;
    unsafe fn on_focus(&self, state: TBool) -> tresult;
    unsafe fn set_frame(&self, frame: *mut c_void) -> tresult;
    unsafe fn can_resize(&self) -> tresult;
    unsafe fn check_size_constraint(&self, rect: *mut ViewRect) -> tresult;
}

#[com_interface("367FAF01-AFA9-4693-8D4D-A2A0ED0882A3")]
pub trait IPlugFrame: IUnknown {
    unsafe fn resize_view(
        &self,
        view: SharedVstPtr<dyn IPlugView>,
        new_size: *mut ViewRect,
    ) -> tresult;
}

#[cfg(target_os = "linux")]
pub mod linux {
    use crate::base::tresult;
    use crate::utils::SharedVstPtr;
    use vst3_com::com_interface;
    use vst3_com::interfaces::iunknown::IUnknown;

    pub type TimerInterval = u64;
    pub type FileDescriptor = i32;

    #[com_interface("561E65C9-13A0-496F-813A-2C35654D7983")]
    pub trait IEventHandler: IUnknown {
        unsafe fn on_fd_is_set(&self, fd: FileDescriptor);
    }

    #[com_interface("10BDD94F-4142-4774-821F-AD8FECA72CA9")]
    pub trait ITimerHandler: IUnknown {
        unsafe fn on_timer(&self);
    }

    #[com_interface("18C35366-9776-4F1A-9C5B-83857A871389")]
    pub trait IRunLoop: IUnknown {
        unsafe fn register_event_handler(
            &self,
            h: SharedVstPtr<dyn IEventHandler>,
            fd: FileDescriptor,
        ) -> tresult;
        unsafe fn unregister_event_handler(&self, h: SharedVstPtr<dyn IEventHandler>) -> tresult;
        unsafe fn register_timer(
            &self,
            t: SharedVstPtr<dyn ITimerHandler>,
            ms: TimerInterval,
        ) -> tresult;
        unsafe fn unregister_timer(&self, t: SharedVstPtr<dyn ITimerHandler>) -> tresult;
    }
}
