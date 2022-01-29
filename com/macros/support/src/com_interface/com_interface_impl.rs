use crate::com_interface::{iid, vtable, vtable_macro};

use proc_macro2::TokenStream as HelperTokenStream;
use quote::quote;
use syn::{ItemTrait, TypeParamBound};

pub fn generate(trait_item: &ItemTrait) -> HelperTokenStream {
    let interface_ident = &trait_item.ident;
    let vtable_ident = vtable::ident(&interface_ident.to_string());
    let iid_ident = iid::ident(interface_ident);
    let vtable_macro = vtable_macro::ident(interface_ident);
    let parent = if let Some(TypeParamBound::Trait(t)) = trait_item.supertraits.first() {
        quote! { #t }
    } else {
        quote! { #interface_ident }
    };

    quote! {
        unsafe impl vst3_com::ComInterface for dyn #interface_ident {
            type VTable = #vtable_ident;
            type Super = dyn #parent;
            const IID: vst3_com::sys::IID = #iid_ident;

        }

        impl <C: #interface_ident> vst3_com::ProductionComInterface<C> for dyn #interface_ident {
            fn vtable<O: vst3_com::offset::Offset>() -> Self::VTable {
                #vtable_macro!(C, O)
            }
        }
    }
}
