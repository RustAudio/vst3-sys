use crate::base::{tresult, FIDString};
use crate::vst::{ParamID, UnitID};
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

#[allow(non_snake_case)]
pub mod FunctionNameType {
    pub const kStringStereoTRS: &[u8; 8usize] = b"Trl Trr\0";
    pub const kCompGainReduction: &'static [u8] = b"Comp:GainReduction\0";
    pub const kCompGainReductionMax: &'static [u8] = b"Comp:GainReductionMax\0";
    pub const kCompGainReductionPeakHold: &'static [u8] = b"Comp:GainReductionPeakHold\0";
    pub const kCompResetGainReductionMax: &'static [u8] = b"Comp:ResetGainReductionMax\0";
    pub const kLowLatencyMode: &'static [u8] = b"LowLatencyMode\0";
    pub const kRandomize: &'static [u8] = b"Randomize\0";
    pub const kWetDryMix: &'static [u8] = b"WetDryMix\0";
}

#[com_interface("6D21E1DC-9119-9D4B-A2A0-2FEF6C1AE55C")]
pub trait IParameterFunctionName: IUnknown {
    unsafe fn get_parameter_id_from_function_name(
        &self,
        unit_id: UnitID,
        function_name: FIDString,
        parameter_id: ParamID,
    ) -> tresult;
}
