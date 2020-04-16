use crate::base::tresult;
use com::com_interface;
use com::interfaces::iunknown::IUnknown;

#[com_interface("8AE54FDA-E930-46B9-A285-55BCDC98E21E")]
pub trait IPrefetchableSupport: IUnknown {
    unsafe fn get_pretchable_support(&self, prefetchable: *mut u32) -> tresult;
}
