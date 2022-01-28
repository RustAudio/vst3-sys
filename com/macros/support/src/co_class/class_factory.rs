use proc_macro2::{Ident, TokenStream as HelperTokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::{ItemStruct, TypeGenerics};

fn get_iclass_factory_interface_ident() -> Ident {
    format_ident!("IClassFactory")
}

pub fn get_class_factory_base_interface_idents() -> Vec<Ident> {
    vec![get_iclass_factory_interface_ident()]
}

pub fn get_class_factory_aggr_map() -> HashMap<Ident, Vec<Ident>> {
    HashMap::new()
}

// We manually generate a ClassFactory without macros, otherwise
// it leads to an infinite loop.
pub fn generate(struct_item: &ItemStruct) -> HelperTokenStream {
    // Manually define base_interface_idents and aggr_map usually obtained by
    // parsing attributes.

    let base_interface_idents = get_class_factory_base_interface_idents();
    let aggr_map = get_class_factory_aggr_map();

    let struct_ident = &struct_item.ident;
    let (class_factory_impl_generics, class_factory_ty_generics, class_factory_where_clause) =
        struct_item.generics.split_for_impl();
    let class_factory_ident = crate::utils::class_factory_ident(&struct_ident);

    let struct_definition = gen_class_factory_struct_definition(&class_factory_ident);
    let lock_server = gen_lock_server();
    let iunknown_impl = gen_iunknown_impl(
        &base_interface_idents,
        &aggr_map,
        &class_factory_ident,
        &class_factory_ty_generics,
    );
    let class_factory_impl = gen_class_factory_impl(
        &base_interface_idents,
        &class_factory_ident,
        &class_factory_ty_generics,
    );

    quote! {
        #struct_definition

        impl #class_factory_impl_generics vst3_com::interfaces::IClassFactory for #class_factory_ident #class_factory_ty_generics #class_factory_where_clause {
            unsafe fn create_instance(
                &self,
                aggr: *mut *const <dyn vst3_com::interfaces::iunknown::IUnknown as vst3_com::ComInterface>::VTable,
                riid: *const vst3_com::sys::IID,
                ppv: *mut *mut std::ffi::c_void,
            ) -> vst3_com::sys::HRESULT {
                // Bringing trait into scope to access IUnknown methods.
                use vst3_com::interfaces::iunknown::IUnknown;

                if !aggr.is_null() {
                    return vst3_com::sys::CLASS_E_NOAGGREGATION;
                }

                let mut instance = #struct_ident::new();
                instance.add_ref();
                let hr = instance.query_interface(riid, ppv);
                instance.release();

                core::mem::forget(instance);
                hr
            }

            #lock_server
        }

        #iunknown_impl

        #class_factory_impl
    }
}

// Can't use gen_base_fields here, since user might not have imported IClassFactory.
pub fn gen_class_factory_struct_definition(class_factory_ident: &Ident) -> HelperTokenStream {
    let ref_count_field = super::com_struct::gen_ref_count_field();
    let interface_ident = get_iclass_factory_interface_ident();
    let vptr_field_ident = crate::utils::vptr_field_ident(&interface_ident);
    quote! {
        #[repr(C)]
        pub struct #class_factory_ident {
            #vptr_field_ident: *const <dyn vst3_com::interfaces::iclass_factory::IClassFactory as vst3_com::ComInterface>::VTable,
            #ref_count_field
        }
    }
}

pub fn gen_lock_server() -> HelperTokenStream {
    quote! {
        // TODO: Implement correctly
        unsafe fn lock_server(&self, _increment: vst3_com::sys::BOOL) -> vst3_com::sys::HRESULT {
            vst3_com::sys::S_OK
        }
    }
}

pub fn gen_iunknown_impl<S: ::std::hash::BuildHasher>(
    base_interface_idents: &[Ident],
    aggr_map: &HashMap<Ident, Vec<Ident>, S>,
    class_factory_ident: &Ident,
    ty_generics: &TypeGenerics,
) -> HelperTokenStream {
    let query_interface = gen_query_interface();
    let add_ref = super::iunknown_impl::gen_add_ref();
    let release = gen_release(
        &base_interface_idents,
        &aggr_map,
        class_factory_ident,
        ty_generics,
    );
    quote! {
        impl vst3_com::interfaces::IUnknown for #class_factory_ident {
            #query_interface
            #add_ref
            #release
        }
    }
}

pub fn gen_release<S: ::std::hash::BuildHasher>(
    base_interface_idents: &[Ident],
    aggr_map: &HashMap<Ident, Vec<Ident>, S>,
    struct_ident: &Ident,
    ty_generics: &TypeGenerics,
) -> HelperTokenStream {
    let ref_count_ident = crate::utils::ref_count_ident();

    let release_decrement = super::iunknown_impl::gen_release_decrement(&ref_count_ident);
    let release_assign_new_count_to_var = super::iunknown_impl::gen_release_assign_new_count_to_var(
        &ref_count_ident,
        &ref_count_ident,
    );
    let release_new_count_var_zero_check =
        super::iunknown_impl::gen_new_count_var_zero_check(&ref_count_ident);
    let release_drops = super::iunknown_impl::gen_release_drops(
        base_interface_idents,
        aggr_map,
        struct_ident,
        ty_generics,
    );

    quote! {
        unsafe fn release(&self) -> u32 {
            use vst3_com::interfaces::iclass_factory::IClassFactory;

            #release_decrement
            #release_assign_new_count_to_var
            if #release_new_count_var_zero_check {
                #release_drops
            }

            #ref_count_ident
        }
    }
}

fn gen_query_interface() -> HelperTokenStream {
    let vptr_field_ident = crate::utils::vptr_field_ident(&get_iclass_factory_interface_ident());

    quote! {
        unsafe fn query_interface(&self, riid: *const vst3_com::sys::IID, ppv: *mut *mut std::ffi::c_void) -> vst3_com::sys::HRESULT {
            // Bringing trait into scope to access add_ref method.
            use vst3_com::interfaces::iunknown::IUnknown;

            let riid = &*riid;
            if riid == &<dyn vst3_com::interfaces::iunknown::IUnknown as vst3_com::ComInterface>::IID || riid == &<dyn vst3_com::interfaces::iclass_factory::IClassFactory as vst3_com::ComInterface>::IID {
                *ppv = &self.#vptr_field_ident as *const _ as *mut std::ffi::c_void;
                self.add_ref();
                vst3_com::sys::NOERROR
            } else {
                *ppv = std::ptr::null_mut::<std::ffi::c_void>();
                vst3_com::sys::E_NOINTERFACE
            }
        }
    }
}

pub fn gen_class_factory_impl(
    base_interface_idents: &[Ident],
    class_factory_ident: &Ident,
    class_factory_ty_generics: &TypeGenerics,
) -> HelperTokenStream {
    let ref_count_field = super::com_struct_impl::gen_allocate_ref_count_field();
    let base_fields = super::com_struct_impl::gen_allocate_base_fields(base_interface_idents);
    let base_inits = super::com_struct_impl::gen_allocate_base_inits(
        class_factory_ident,
        class_factory_ty_generics,
        base_interface_idents,
    );

    quote! {
        impl #class_factory_ident {
            pub(crate) fn new() -> Box<#class_factory_ident> {
                use vst3_com::interfaces::iclass_factory::IClassFactory;

                // allocate directly since no macros generated an `allocate` function
                #base_inits

                let out = #class_factory_ident {
                    #base_fields
                    #ref_count_field
                };
                Box::new(out)
            }
        }
    }
}
