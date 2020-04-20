use crate::base::tresult;
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PhysicalUIMap {
    physical_ui_type_id: u32,
    note_expression_type_id: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PhysicalUIMapList {
    count: u32,
    map: *mut PhysicalUIMap,
}

#[com_interface("B03078FF-94D2-4AC8-90CC-D303D4133324")]
pub trait INoteExpressionPhysicalUIMapping: IUnknown {
    unsafe fn get_physical_ui_mapping(
        &self,
        bus_index: i32,
        channel: i16,
        list: *mut PhysicalUIMapList,
    ) -> tresult;
}
