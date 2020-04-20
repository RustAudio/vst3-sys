use crate::base::{char8, tresult};
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RepresentationInfo {
    pub vendor: [char8; 64],
    pub name: [char8; 64],
    pub version: [char8; 64],
    pub host: [char8; 64],
}

#[com_interface("A81A0471-48C3-4DC4-AC30-C9E13C8393D5")]
pub trait IXmlRepresentationController: IUnknown {
    unsafe fn get_xml_representation(&self, info: *mut RepresentationInfo) -> tresult;
}

//todo: constants in ivstrepresentation.rs
