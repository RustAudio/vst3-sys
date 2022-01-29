//! Utilities for consumers of the raw API
use vst3_com::{ComInterface, ComPtr};

/// A thin wrapper around a raw pointer to a vtable. Used in traits that return pointers to instances.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct SharedVstPtr<I: ComInterface + ?Sized> {
    inst: *mut *mut <I as ComInterface>::VTable,
}

impl<I: ComInterface + ?Sized> SharedVstPtr<I> {
    pub fn as_raw_mut(&mut self) -> *mut *mut <I as ComInterface>::VTable {
        self.inst
    }
    /// check if the underlying pointer is null
    pub fn is_null(&self) -> bool {
        self.inst.is_null()
    }

    /// Promote the pointer to a reference count, returns `None` if the pointer is null.
    pub fn upgrade(&self) -> Option<ComPtr<I>> {
        if self.inst.is_null() {
            None
        } else {
            // Safety: we only guarantee the pointer is not null, if the code that allocated the pointer is flawed  it could still point to garbage.
            unsafe { Some(ComPtr::new(self.inst)) }
        }
    }
}
