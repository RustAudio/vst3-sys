use crate::base::tresult;
use crate::vst::ivstnoteexpression::NoteExpressionTypeID;
use vst3_com::com_interface;
use vst3_com::interfaces::iunknown::IUnknown;

pub enum PhysicalUITypeID
{
	kPUIXMovement = 0,
	kPUIYMovement = 1,
	kPUIPressure = 2,
	kPUITypeCount = 3,
	kInvalidPUITypeID = 0xFFFFFFFF
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PhysicalUIMap {
    pub physical_ui_type_id: PhysicalUITypeID,
    pub note_expression_type_id: NoteExpressionTypeID,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PhysicalUIMapList {
    pub count: u32,
    pub map: *mut PhysicalUIMap,
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
