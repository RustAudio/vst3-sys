use proc_macro2::TokenStream as HelperTokenStream;
use quote::quote;
use syn::ItemStruct;

// We manually generate a ClassFactory without macros, otherwise
// it leads to an infinite loop.
pub fn generate(struct_item: &ItemStruct) -> HelperTokenStream {
    let base_interface_idents =
        crate::co_class::class_factory::get_class_factory_base_interface_idents();
    let aggr_map = crate::co_class::class_factory::get_class_factory_aggr_map();

    let struct_ident = &struct_item.ident;
    let class_factory_ident = crate::utils::class_factory_ident(&struct_ident);
    let (class_factory_impl_generics, class_factory_ty_generics, class_factory_where_clause) =
        struct_item.generics.split_for_impl();

    let struct_definition =
        crate::co_class::class_factory::gen_class_factory_struct_definition(&class_factory_ident);
    let lock_server = crate::co_class::class_factory::gen_lock_server();
    let iunknown_impl = crate::co_class::class_factory::gen_iunknown_impl(
        &base_interface_idents,
        &aggr_map,
        &class_factory_ident,
        &class_factory_ty_generics,
    );
    let class_factory_impl = crate::co_class::class_factory::gen_class_factory_impl(
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

                let riid = unsafe { &*riid };

                if !aggr.is_null() && riid != &<dyn vst3_com::interfaces::iunknown::IUnknown as vst3_com::ComInterface>::IID {
                    unsafe {
                        *ppv = std::ptr::null_mut::<std::ffi::c_void>();
                    }
                    return vst3_com::sys::E_INVALIDARG;
                }

                let mut instance = #struct_ident::new();

                // This check has to be here because it can only be done after object
                // is allocated on the heap (address of nonDelegatingUnknown fixed)
                instance.set_iunknown(aggr);

                // As an aggregable object, we have to add_ref through the
                // non-delegating IUnknown on creation. Otherwise, we might
                // add_ref the outer object if aggregated.
                instance.inner_add_ref();
                let hr = instance.inner_query_interface(riid, ppv);
                instance.inner_release();

                core::mem::forget(instance);
                hr
            }

            #lock_server
        }

        #iunknown_impl

        #class_factory_impl
    }
}
