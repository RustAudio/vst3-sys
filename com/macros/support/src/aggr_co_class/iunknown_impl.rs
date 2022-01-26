use proc_macro2::TokenStream as HelperTokenStream;
use quote::quote;
use syn::ItemStruct;

/// For an aggregable COM object, the default IUnknown implementation is
/// always the delegating IUnknown implementation. This will always
/// delegate to the interface pointer at __iunknown_to_use.
pub fn generate(struct_item: &ItemStruct) -> HelperTokenStream {
    let struct_ident = &struct_item.ident;
    let (impl_generics, ty_generics, where_clause) = struct_item.generics.split_for_impl();

    let iunknown_to_use_field_ident = crate::utils::iunknown_to_use_field_ident();
    let ptr_casting = quote! { as *mut _ };

    quote!(
        impl #impl_generics vst3_com::interfaces::IUnknown for #struct_ident #ty_generics #where_clause {
            unsafe fn query_interface(
                &self,
                riid: *const vst3_com::sys::IID,
                ppv: *mut *mut std::ffi::c_void
            ) -> vst3_com::sys::HRESULT {
                let iunknown_to_use = vst3_com::ComPtr::<dyn vst3_com::interfaces::IUnknown>::new(self.#iunknown_to_use_field_ident #ptr_casting);
                iunknown_to_use.query_interface(riid, ppv)
            }

            unsafe fn add_ref(&self) -> u32 {
                let iunknown_to_use  = unsafe { vst3_com::ComPtr::<dyn vst3_com::interfaces::IUnknown>::new(self.#iunknown_to_use_field_ident #ptr_casting) };
                iunknown_to_use.add_ref()
            }

            unsafe fn release(&self) -> u32 {
                let iunknown_to_use = vst3_com::ComPtr::<dyn vst3_com::interfaces::IUnknown>::new(self.#iunknown_to_use_field_ident #ptr_casting);
                iunknown_to_use.release()
            }
        }
    )
}
