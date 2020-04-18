use crate::base::tresult;
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

#[com_interface("6B2449CC-4197-40B5-AB3C-79DAC5FE5C86")]
pub trait IMidiLearn: IUnknown {
    unsafe fn on_live_midi_controller_input(
        &self,
        bus_idx: i32,
        channel: i16,
        midi_cc: i16,
    ) -> tresult;
}
