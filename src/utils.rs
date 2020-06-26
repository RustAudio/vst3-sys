//! Utilities for consumers of the raw API
use vst3_com::{ComInterface, ComRc};

/// A thin wrapper around a raw pointer to a vtable. Used in traits that return pointers to instances.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct VstPtr<I: ComInterface + ?Sized> {
    inst: *mut *mut <I as ComInterface>::VTable,
}

impl<I: ComInterface + ?Sized> VstPtr<I> {
    pub fn is_null(&self) -> bool {
        self.inst.is_null()
    }
    pub fn upgrade(&self) -> Option<ComRc<I>> {
        if self.inst.is_null() {
            None
        } else {
            unsafe { Some(ComRc::from_raw(self.inst)) }
        }
    }
}
