//! A helper crate for consuming and producing COM interfaces.
//!
//! # Example
//!
//! To work with a COM interface it must first be declared:
//!
//! ```rust,no_run
//! /// Define an IAnimal interface wit
//! #[vst3_com::com_interface("EFF8970E-C50F-45E0-9284-291CE5A6F771")]
//! pub trait IAnimal: vst3_com::interfaces::IUnknown {
//!     unsafe fn eat(&self) -> vst3_com::sys::HRESULT;
//! }
//! ```
//!

#![deny(missing_docs)]

mod ptr;
mod rc;

pub mod interfaces;
use interfaces::IUnknown;

#[doc(hidden)]
pub mod offset;

#[cfg(windows)]
pub mod runtime;
pub mod sys;

pub use ptr::ComPtr;
pub use rc::ComRc;

pub use std::ffi::c_void;
#[doc(inline)]
pub use sys::{CLSID, IID};

/// pointer to an IID
pub type REFIID = *const IID;

/// A COM compliant interface
///
/// # Safety
///
/// The trait or struct implementing this trait must provide a valid vtable as the
/// associated VTable type. A vtable is valid if:
/// * it is `#[repr(C)]`
/// * the type only contains `extern "system" fn" definitions
pub unsafe trait ComInterface: IUnknown + 'static {
    /// A COM compatible V-Table
    type VTable;
    /// The interface that this interface inherits from
    type Super: ComInterface + ?Sized;
    /// The associated id for this interface
    const IID: IID;

    /// Check whether a given IID is in the inheritance hierarchy of this interface
    fn is_iid_in_inheritance_chain(riid: &IID) -> bool {
        // println!("this: {:X}-{:X}-{:X}-{:?}", Self::IID.data1, Self::IID.data2, Self::IID.data3, Self::IID.data4);
        // println!("that: {:X}-{:X}-{:X}-{:?}", riid.data1, riid.data2, riid.data3, riid.data4);
        riid == &Self::IID
            || (Self::IID != <dyn IUnknown as ComInterface>::IID
                && <Self::Super as ComInterface>::is_iid_in_inheritance_chain(riid))
    }
}

/// A COM compliant class
///
/// # Safety
///
/// The implementing struct must have the following properties:
/// * it is `#[repr(C)]`
/// * The first fields of the struct are pointers to the backing VTables for
/// each of the COM Interfaces the class implements
pub unsafe trait CoClass: IUnknown {}

/// A COM interface that will be exposed in a COM server
pub trait ProductionComInterface<T: IUnknown>: ComInterface {
    /// Get the vtable for a particular COM interface
    fn vtable<O: offset::Offset>() -> Self::VTable;
}

#[doc(hidden)]
#[macro_export]
macro_rules! vtable {
    ($class:ident: $interface:ident, $offset:path) => {
        <dyn $interface as $crate::ProductionComInterface<$class>>::vtable::<$offset>();
    };
}

vst3_com_macros::declare_offsets!();
#[doc(hidden)]
pub use vst3_com_macros::{co_class, com_interface, VTable};

// this allows for the crate to refer to itself as `com` to keep macros consistent
// whether they are used by some other crate or internally
#[doc(hidden)]
extern crate self as com;
