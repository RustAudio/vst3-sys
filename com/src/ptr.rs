use std::ffi::c_void;
use std::ptr::NonNull;

use crate::sys::S_OK;
use crate::{interfaces::IUnknown, ComInterface, IID};

/// A smart pointer for VST3 interface objects. You'll find these objects in two different places in
/// the VST3 API: passed as a parameter to a method (owned), and returned by a method (shared). In
/// the first situation, the object already has a reference count of 1, and the caller should take
/// ownership of it. In the second situation, the callee should add a reference if it decides to use
/// the pointer. The semantics of these two situations are encoded in the [VstPtr::owned] and
/// [VstPtr::shared] function respectively.
#[repr(transparent)]
pub struct VstPtr<T: ComInterface + ?Sized> {
    ptr: NonNull<*mut <T as ComInterface>::VTable>,
}

/// [VstPtr], but without any lifetime management. This is only used for the parts of the API where
/// the host may not have implemented the correct lifetime management features, such as in the
/// process context. You should never use this yourself.
#[repr(transparent)]
#[derive(Clone)]
pub struct RawVstPtr<T: ComInterface + ?Sized> {
    ptr: NonNull<*mut <T as ComInterface>::VTable>,
}

impl<T: ComInterface + ?Sized> VstPtr<T> {
    /// Adopt a pointer returned by a VST3 method call. This will return a `None` if the pointer is
    /// a null pointer. This function must be called whenever a method returns an interface opinter,
    /// even if you do not plan on using the value. Otherwise the object will leak.
    ///
    /// The object will be released when the last reference to it gets dropped.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid interface pointer for interface `T`. An interface
    /// pointer as the name suggests points to an interface struct. A valid
    /// interface is itself trivial castable to a `*mut T::VTable`. In other words,
    /// `ptr` should also be equal to `*mut *mut T::VTable`
    ///
    /// `ptr` must live for at least as long as the `VstPtr`. The underlying
    /// COM interface is assumed to correctly implement AddRef and Release such that
    /// the interface will be valid as long as AddRef has been called more times than
    /// Release.
    pub unsafe fn owned(ptr: *mut *mut <T as ComInterface>::VTable) -> Option<Self> {
        Some(Self {
            ptr: NonNull::new(ptr)?,
        })
    }

    /// Add another reference for a VST3 interface object passed as an argument to a method call.
    /// This will return a `None` if the pointer is a null pointer.
    ///
    /// The object will be released when the last reference to it gets dropped.
    ///
    /// # Safety
    ///
    /// See the safety notes in [Self::owned].
    pub unsafe fn shared(ptr: *mut *mut <T as ComInterface>::VTable) -> Option<Self> {
        let vst_ptr = Self {
            ptr: NonNull::new(ptr)?,
        };
        vst_ptr.add_ref();

        Some(vst_ptr)
    }

    /// Get the underlying interface pointer. This pointer is only guarnteed to live for as long as
    /// the current `VstPtr` is alive.
    pub fn as_ptr(&self) -> *mut *mut <T as ComInterface>::VTable {
        self.ptr.as_ptr()
    }

    /// A smart cast from this interface to another interface implemented by the same object. If the
    /// object does not implement the interface, then this will return a `None`.
    pub fn cast<I: ComInterface + ?Sized>(&self) -> Option<VstPtr<I>> {
        let mut obj = std::ptr::null_mut::<c_void>();
        let result = unsafe { self.query_interface(&I::IID as *const IID, &mut obj) };
        if result == S_OK {
            // There's no way to guarentee that the returned object actually impelemnts this
            // interface, but you got to have a little faith
            unsafe { VstPtr::owned(obj as *mut *mut _) }
        } else {
            None
        }
    }
}

impl<T: ComInterface + ?Sized> Clone for VstPtr<T> {
    fn clone(&self) -> Self {
        unsafe {
            VstPtr::shared(self.ptr.as_ptr())
                .expect("Wait, it's all null pointers? Always has been.")
        }
    }
}

impl<T: ComInterface + ?Sized> Drop for VstPtr<T> {
    fn drop(&mut self) {
        unsafe {
            self.release();
        }
    }
}

impl<T: ComInterface + ?Sized> RawVstPtr<T> {
    /// Construct an unmanaged VST interface pointer from a raw pointer. This will return a `None`
    /// if the pointer is a null pointer. Consider using [VstPtr] instead.
    ///
    /// # Safety
    ///
    /// See the safety notes in [VstPtr::owned].
    pub unsafe fn new(ptr: *mut *mut <T as ComInterface>::VTable) -> Option<Self> {
        Some(Self {
            ptr: NonNull::new(ptr)?,
        })
    }

    /// Get the underlying interface pointer. This pointer is only guarnteed to live for as long as
    /// the current `VstPtr` is alive.
    pub fn as_ptr(&self) -> *mut *mut <T as ComInterface>::VTable {
        self.ptr.as_ptr()
    }
}
