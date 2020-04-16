use crate::base::{char8, tresult};
use com::com_interface;
use com::interfaces::iunknown::IUnknown;

#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct RepresentationInfo {
    vendor: [char8; 64],
    name: [char8; 64],
    version: [char8; 64],
    host: [char8; 64],
}

#[com_interface("A81A0471-48C3-4DC4-AC30-C9E13C8393D5")]
pub trait IXmlRepresentationController: IUnknown {
    unsafe fn get_xml_representation(&self, info: *mut RepresentationInfo) -> tresult;
}

//todo: constants in ivstrepresentation.rs
