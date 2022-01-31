//! Utilities for working with VST3 interface objects.

use vst3_com::{ComInterface, RawVstPtr};

pub use vst3_com::VstPtr;

/// A [VstPtr] with shared semantics. Used only as a function parameter in the interface traits.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SharedVstPtr<I: ComInterface + ?Sized> {
    ptr: *mut *mut <I as ComInterface>::VTable,
}

/// A [VstPtr]-like object without any lifetime management. Used only as a field in structs
/// containing interface pointers.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StaticVstPtr<I: ComInterface + ?Sized> {
    ptr: *mut *mut <I as ComInterface>::VTable,
}

impl<I: ComInterface + ?Sized> SharedVstPtr<I> {
    pub fn as_ptr(&mut self) -> *mut *mut <I as ComInterface>::VTable {
        self.ptr
    }

    /// Check if the underlying pointer is null.
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    /// Promote the pointer to a reference count, returns `None` if the pointer is null.
    pub fn upgrade(&self) -> Option<VstPtr<I>> {
        // Safety: we only guarantee the pointer is not null, if the code that allocated the pointer is flawed  it could still point to garbage.
        unsafe { VstPtr::shared(self.ptr) }
    }
}

impl<I: ComInterface + ?Sized> StaticVstPtr<I> {
    pub fn as_ptr(&mut self) -> *mut *mut <I as ComInterface>::VTable {
        self.ptr
    }

    /// Check if the underlying pointer is null.
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    /// Promote the pointer to a reference count, returns `None` if the pointer is null.
    pub fn upgrade(&self) -> Option<RawVstPtr<I>> {
        // Safety: we only guarantee the pointer is not null, if the code that allocated the pointer is flawed  it could still point to garbage.
        unsafe { RawVstPtr::new(self.ptr) }
    }
}
