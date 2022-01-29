use proc_macro2::TokenStream as HelperTokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Ident, ItemStruct, TypeGenerics};

/// Generates the methods that the com struct needs to have. These include:
/// allocate: To initialise the vtables, including the non_delegatingegating_iunknown one.
/// set_iunknown: For Class Objects to set the iunknown to use, for aggregation.
/// inner_iunknown: declare the non_delegatingegating iunknown functions on the com struct.
/// get_class_object: Entry point to obtain the IClassFactory Class Object suited for this class.
/// set_aggregate_*: Functions to initialise aggregation for the group the interface belongs to.
pub fn generate(
    base_interface_idents: &[Ident],
    aggr_map: &HashMap<Ident, Vec<Ident>>,
    struct_item: &ItemStruct,
) -> HelperTokenStream {
    let struct_ident = &struct_item.ident;
    let (impl_generics, ty_generics, where_clause) = struct_item.generics.split_for_impl();

    let allocate_fn = gen_allocate_fn(aggr_map, base_interface_idents, struct_item);
    let set_iunknown_fn = gen_set_iunknown_fn();
    let inner_iunknown_fns =
        gen_inner_iunknown_fns(base_interface_idents, aggr_map, struct_ident, &ty_generics);
    let set_aggregate_fns = crate::co_class::com_struct_impl::gen_set_aggregate_fns(aggr_map);

    quote!(
        impl #impl_generics #struct_ident #ty_generics #where_clause {
            #allocate_fn
            #set_iunknown_fn
            #inner_iunknown_fns
            #set_aggregate_fns
        }
    )
}

/// Function that should only be used by Class Object, to set the
/// object's iunknown_to_use, if the object is going to get aggregated.
fn gen_set_iunknown_fn() -> HelperTokenStream {
    let iunknown_to_use_field_ident = crate::utils::iunknown_to_use_field_ident();
    let non_delegating_iunknown_field_ident = crate::utils::non_delegating_iunknown_field_ident();

    quote!(
        pub(crate) fn set_iunknown(&mut self, aggr: *mut *const <dyn vst3_com::interfaces::iunknown::IUnknown as vst3_com::ComInterface>::VTable) {
            if aggr.is_null() {
                self.#iunknown_to_use_field_ident = &self.#non_delegating_iunknown_field_ident as *const _ as *mut *const <dyn vst3_com::interfaces::iunknown::IUnknown as vst3_com::ComInterface>::VTable;
            } else {
                self.#iunknown_to_use_field_ident = aggr;
            }
        }
    )
}

/// The non-delegating IUnknown implementation for an aggregable object. This will contain
/// the actual IUnknown implementations for the object.
fn gen_inner_iunknown_fns(
    base_interface_idents: &[Ident],
    aggr_map: &HashMap<Ident, Vec<Ident>>,
    struct_ident: &Ident,
    ty_generics: &TypeGenerics,
) -> HelperTokenStream {
    let inner_query_interface = gen_inner_query_interface(base_interface_idents, aggr_map);
    let inner_add_ref = gen_inner_add_ref();
    let inner_release =
        gen_inner_release(base_interface_idents, aggr_map, struct_ident, ty_generics);

    quote!(
        #inner_query_interface
        #inner_add_ref
        #inner_release
    )
}

pub fn gen_inner_add_ref() -> HelperTokenStream {
    let add_ref_implementation = crate::co_class::iunknown_impl::gen_add_ref_implementation();

    quote! {
        pub(crate) fn inner_add_ref(&self) -> u32 {
            #add_ref_implementation
        }
    }
}

pub fn gen_inner_release(
    base_interface_idents: &[Ident],
    aggr_map: &HashMap<Ident, Vec<Ident>>,
    struct_ident: &Ident,
    ty_generics: &TypeGenerics,
) -> HelperTokenStream {
    let ref_count_ident = crate::utils::ref_count_ident();

    let release_decrement = crate::co_class::iunknown_impl::gen_release_decrement(&ref_count_ident);
    let release_assign_new_count_to_var =
        crate::co_class::iunknown_impl::gen_release_assign_new_count_to_var(
            &ref_count_ident,
            &ref_count_ident,
        );
    let release_new_count_var_zero_check =
        crate::co_class::iunknown_impl::gen_new_count_var_zero_check(&ref_count_ident);
    let release_drops = crate::co_class::iunknown_impl::gen_release_drops(
        base_interface_idents,
        aggr_map,
        struct_ident,
        ty_generics,
    );
    let non_delegating_iunknown_drop = gen_non_delegating_iunknown_drop();

    quote! {
        unsafe fn inner_release(&self) -> u32 {
            #release_decrement
            #release_assign_new_count_to_var
            if #release_new_count_var_zero_check {
                #non_delegating_iunknown_drop
                #release_drops
            }

            #ref_count_ident
        }
    }
}

fn gen_non_delegating_iunknown_drop() -> HelperTokenStream {
    let non_delegating_iunknown_field_ident = crate::utils::non_delegating_iunknown_field_ident();
    quote!(
        Box::from_raw(self.#non_delegating_iunknown_field_ident as *mut <dyn vst3_com::interfaces::iunknown::IUnknown as vst3_com::ComInterface>::VTable);
    )
}

/// Non-delegating query interface
fn gen_inner_query_interface(
    base_interface_idents: &[Ident],
    aggr_map: &HashMap<Ident, Vec<Ident>>,
) -> HelperTokenStream {
    let non_delegating_iunknown_field_ident = crate::utils::non_delegating_iunknown_field_ident();

    // Generate match arms for implemented interfaces
    let base_match_arms =
        crate::co_class::iunknown_impl::gen_base_match_arms(base_interface_idents);

    // Generate match arms for aggregated interfaces
    let aggr_match_arms = crate::co_class::iunknown_impl::gen_aggregate_match_arms(aggr_map);

    quote!(
        pub(crate) fn inner_query_interface(&self, riid: *const vst3_com::sys::IID, ppv: *mut *mut std::ffi::c_void) -> vst3_com::sys::HRESULT {
            unsafe {
                let riid = &*riid;

                if riid == &vst3_com::interfaces::iunknown::IID_IUNKNOWN {
                    *ppv = &self.#non_delegating_iunknown_field_ident as *const _ as *mut std::ffi::c_void;
                } #base_match_arms #aggr_match_arms else {
                    *ppv = std::ptr::null_mut::<std::ffi::c_void>();
                    return vst3_com::sys::E_NOINTERFACE;
                }

                self.inner_add_ref();
                vst3_com::sys::NOERROR
            }
        }
    )
}

/// For an aggregable object, we have to do more work here. We need to
/// instantiate the non-delegating IUnknown vtable. The unsafe extern "system"
/// methods belonging to the non-delegating IUnknown vtable are also defined here.
fn gen_allocate_fn(
    aggr_map: &HashMap<Ident, Vec<Ident>>,
    base_interface_idents: &[Ident],
    struct_item: &ItemStruct,
) -> HelperTokenStream {
    let struct_ident = &struct_item.ident;
    let (_, ty_generics, _) = struct_item.generics.split_for_impl();

    let base_inits = crate::co_class::com_struct_impl::gen_allocate_base_inits(
        struct_ident,
        &ty_generics,
        base_interface_idents,
    );

    // Allocate function signature
    let allocate_parameters =
        crate::co_class::com_struct_impl::gen_allocate_function_parameters_signature(struct_item);

    // Syntax for instantiating the fields of the struct.
    let base_fields =
        crate::co_class::com_struct_impl::gen_allocate_base_fields(base_interface_idents);
    let ref_count_field = crate::co_class::com_struct_impl::gen_allocate_ref_count_field();
    let user_fields = crate::co_class::com_struct_impl::gen_allocate_user_fields(struct_item);
    let aggregate_fields =
        crate::co_class::com_struct_impl::gen_allocate_aggregate_fields(aggr_map);

    // Aggregable COM struct specific fields
    let iunknown_to_use_field_ident = crate::utils::iunknown_to_use_field_ident();
    let non_delegating_iunknown_field_ident = crate::utils::non_delegating_iunknown_field_ident();
    let non_delegating_iunknown_offset = base_interface_idents.len();

    quote!(
        fn allocate(#allocate_parameters) -> Box<#struct_ident #ty_generics> {
            // Non-delegating methods.
            unsafe extern "system" fn non_delegatingegating_query_interface(
                this: *mut *const <dyn vst3_com::interfaces::iunknown::IUnknown as vst3_com::ComInterface>::VTable,
                riid: *const vst3_com::sys::IID,
                ppv: *mut *mut std::ffi::c_void,
            ) -> HRESULT {
                let this = this.sub(#non_delegating_iunknown_offset) as *mut #struct_ident;
                (*this).inner_query_interface(riid, ppv)
            }

            unsafe extern "system" fn non_delegatingegating_add_ref(
                this: *mut *const <dyn vst3_com::interfaces::iunknown::IUnknown as vst3_com::ComInterface>::VTable,
            ) -> u32 {
                let this = this.sub(#non_delegating_iunknown_offset) as *mut #struct_ident;
                (*this).inner_add_ref()
            }

            unsafe extern "system" fn non_delegatingegating_release(
                this: *mut *const <dyn vst3_com::interfaces::iunknown::IUnknown as vst3_com::ComInterface>::VTable,
            ) -> u32 {
                let this = this.sub(#non_delegating_iunknown_offset) as *mut #struct_ident;
                (*this).inner_release()
            }

            // Rust Parser limitation? Unable to construct associated type directly.
            type __iunknown_vtable_type = <dyn vst3_com::interfaces::iunknown::IUnknown as vst3_com::ComInterface>::VTable;
            let __non_delegating_iunknown_vtable =  __iunknown_vtable_type {
                QueryInterface: non_delegatingegating_query_interface,
                Release: non_delegatingegating_release,
                AddRef: non_delegatingegating_add_ref,
            };
            let #non_delegating_iunknown_field_ident = Box::into_raw(Box::new(__non_delegating_iunknown_vtable));

            #base_inits

            let out = #struct_ident {
                #base_fields
                #non_delegating_iunknown_field_ident,
                #iunknown_to_use_field_ident: std::ptr::null_mut::<*const <dyn vst3_com::interfaces::iunknown::IUnknown as vst3_com::ComInterface>::VTable>(),
                #ref_count_field
                #aggregate_fields
                #user_fields
            };
            Box::new(out)
        }
    )
}
