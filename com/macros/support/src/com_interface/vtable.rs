use super::vptr;
use proc_macro2::{Ident, TokenStream as HelperTokenStream};
use quote::{format_ident, quote};
use std::iter::FromIterator;
use syn::{FnArg, ItemTrait, TraitItem, TraitItemMethod, Type, TypeParamBound};

/// Generate an VTable for an interface trait
///
/// * `interface` is a trait representing a COM interface. This trait must either be
/// IUnknown and have no super traits or some other trait that has a parent trait
pub fn generate(interface: &ItemTrait) -> HelperTokenStream {
    let interface_ident = &interface.ident;
    let vtable_ident = ident(&interface_ident.to_string());
    let base_field = if interface_ident.to_string().to_uppercase() == "IUNKNOWN" {
        assert!(
            interface.supertraits.is_empty(),
            "IUnknown is a reserved interface"
        );
        quote! {}
    } else {
        assert!(
            !(interface.supertraits.len() > 1),
            "Multiple inheritance is not supported in COM interfaces"
        );
        assert!(
            !interface.supertraits.is_empty(),
            "All interfaces must inherit from another COM interface"
        );

        let base_interface_path = match interface.supertraits.first().expect("No supertraits") {
            TypeParamBound::Trait(path) => path,
            _ => panic!("Unhandled super trait typeparambound"),
        };

        let last_ident = &base_interface_path
            .path
            .segments
            .last()
            .expect("Supertrait has empty path")
            .ident;
        let base_field_ident = base_field_ident(&last_ident.to_string());
        quote! {
            pub #base_field_ident: <dyn #base_interface_path as vst3_com::ComInterface>::VTable,
        }
    };
    let methods = gen_vtable_methods(interface);

    quote!(
        #[allow(non_snake_case, missing_docs)]
        #[repr(C)]
        #[derive(vst3_com::VTable)]
        pub struct #vtable_ident {
            #base_field
            #methods
        }
    )
}

pub fn ident(interface_name: &str) -> Ident {
    format_ident!("{}VTable", interface_name)
}

fn base_field_ident(base_interface_name: &str) -> Ident {
    format_ident!("{}_base", crate::utils::camel_to_snake(base_interface_name))
}

fn gen_vtable_methods(interface: &ItemTrait) -> HelperTokenStream {
    let mut methods: Vec<HelperTokenStream> = Vec::new();
    for trait_item in &interface.items {
        match trait_item {
            TraitItem::Method(m) => methods.push(gen_vtable_method(&interface.ident, m)),
            _ => panic!("Interface traits currently only support methods"),
        };
    }

    quote!(
        #(#methods)*
    )
}

fn gen_vtable_method(interface_ident: &Ident, method: &TraitItemMethod) -> HelperTokenStream {
    assert!(
        method.sig.unsafety.is_some(),
        "COM Interface methods must be declared unsafe"
    );
    let method_ident = format_ident!(
        "{}",
        crate::utils::snake_to_camel(&method.sig.ident.to_string())
    );
    let vtable_function_signature = gen_vtable_function_signature(interface_ident, method);

    quote!(
        pub #method_ident: #vtable_function_signature,
    )
}

fn gen_vtable_function_signature(
    interface_ident: &Ident,
    method: &TraitItemMethod,
) -> HelperTokenStream {
    let params = gen_raw_params(interface_ident, method);
    let return_type = &method.sig.output;

    quote!(
        unsafe extern "system" fn(#params) #return_type
    )
}

fn gen_raw_params(interface_ident: &Ident, method: &TraitItemMethod) -> HelperTokenStream {
    let mut params = Vec::new();
    let vptr_ident = vptr::ident(&interface_ident.to_string());

    for param in method.sig.inputs.iter() {
        match param {
            FnArg::Receiver(s) => {
                assert!(
                    s.reference.is_some(),
                    "COM interface methods cannot take ownership of self"
                );
                assert!(
                    s.mutability.is_none(),
                    "COM interface methods cannot take mutable reference to self"
                );
                params.push(quote!(
                    *mut #vptr_ident,
                ));
            }
            FnArg::Typed(t) => {
                params.push(gen_raw_type(&*t.ty));
            }
        }
    }

    HelperTokenStream::from_iter(params)
}

fn gen_raw_type(t: &Type) -> HelperTokenStream {
    match t {
        Type::Array(_n) => panic!("Array type unhandled!"),
        Type::BareFn(_n) => panic!("BareFn type unhandled!"),
        Type::Group(_n) => panic!("Group type unhandled!"),
        Type::ImplTrait(_n) => panic!("ImplTrait type unhandled!"),
        Type::Infer(_n) => panic!("Infer type unhandled!"),
        Type::Macro(_n) => panic!("TypeMacro type unhandled!"),
        Type::Never(_n) => panic!("TypeNever type unhandled!"),
        Type::Paren(_n) => panic!("Paren type unhandled!"),
        Type::Path(_n) => quote!(#t,),
        Type::Ptr(_n) => quote!(#t,),
        Type::Reference(_n) => panic!("Reference type unhandled!"),
        Type::Slice(_n) => panic!("Slice type unhandled!"),
        Type::TraitObject(_n) => panic!("TraitObject type unhandled!"),
        Type::Tuple(_n) => panic!("Tuple type unhandled!"),
        Type::Verbatim(_n) => panic!("Verbatim type unhandled!"),
        _ => panic!("Rest unhandled!"),
    }
}
