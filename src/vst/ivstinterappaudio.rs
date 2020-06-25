use crate::base::tresult;
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

// Defined elsewhere
struct ViewRect {};
struct Event {};

#[com_interface("FDC8CDE2-28AE-D45B-68DF-415E0CE5743D")]
pub trait IInterAppAudioHost: IUnknown {
    unsafe fn get_screen_size(&self, size: *mut ViewRect, scale: *mut f32) -> tresult;
    unsafe fn connected_to_host(&self) -> tresult;
    unsafe fn switch_to_host(&self) -> tresult;
    unsafe fn send_remote_control_event(&self, event: u32) -> tresult;
    unsafe fn get_host_icon(&self, icon: *mut *mut c_void) -> tresult;
    unsafe fn schedule_event_from_ui(&self, event: &mut Event) -> tresult;
    unsafe fn create_preset_manager(&self, cid: *const IID) -> Option<*mut IInterAppAudioPresetManager>;
    unsafe fn show_settings_view(&self) -> tresult;
}

#[com_interface("CFD5D6D7-95B0-B50D-5FC2-4AA16020C72D")]
pub trait IInterAppAudioConnectionNotification: IUnknown {
    unsafe fn on_inter_app_audio_connection_state_change(&self, new_state: TBool) -> tresult;
}

#[com_interface("DDEF3FC9-B4B3-809A-46C9-4E1DADE6FCC4")]
pub trait IInterAppAudioPresetManager: IUnknown {
    unsafe fn run_load_preset_browser(&self) -> tresult;
	  unsafe fn run_save_preset_browser(&self) -> tresult;
	  unsafe fn load_next_preset(&self) -> tresult;
	  unsafe fn load_previous_preset(&self) -> tresult;
}
