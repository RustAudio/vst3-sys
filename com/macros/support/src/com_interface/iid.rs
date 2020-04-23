use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as HelperTokenStream};
use quote::{format_ident, quote};
use syn::LitInt;

pub fn generate(macro_attr: &TokenStream, interface_ident: &Ident) -> HelperTokenStream {
    let iid_string: syn::LitStr =
        syn::parse(macro_attr.clone()).expect("[com_interface] parameter must be a GUID string");
    let iid_value = iid_string.value();
    assert!(
        iid_value.len() == 36,
        "IIDs must be exactly 36 characters long"
    );

    let iid_ident = ident(interface_ident);
    let iid_value = iid_value.as_str();
    let delimited: Vec<&str> = iid_value.split('-').collect();
    assert!(
        delimited.len() == 5,
        "IIDs must have 5 parts separate by '-'s"
    );

    assert!(
        delimited[0].len() == 8,
        "The first part of the IID must be 8 characters long, but it is {} characters long",
        delimited[0].len()
    );

    assert!(
        delimited[1].len() == 4,
        "The second part of the IID must be 4 characters long, but it is {} characters long",
        delimited[1].len()
    );

    assert!(
        delimited[2].len() == 4,
        "The third part of the IID must be 4 characters long, but it is {} characters long",
        delimited[2].len()
    );

    assert!(
        delimited[3].len() == 4,
        "The fourth part of the IID must be 4 characters long, but it is {} characters long",
        delimited[3].len()
    );

    assert!(
        delimited[4].len() == 12,
        "The fifth part of the IID must be 12 characters long, but it is {} characters long",
        delimited[4].len()
    );

    let flat = delimited
        .into_iter()
        .flat_map(|s| s.chars())
        .collect::<Vec<_>>();
    let bytes = (0..32).step_by(2).map(|idx| {
        let mut chars = ['0', 'x', '\0', '\0'];
        chars[2] = flat[idx];
        chars[3] = flat[idx + 1];
        let string = chars.iter().collect::<String>();
        LitInt::new(&string, Span::call_site())
    });

    quote!(
        #[allow(non_upper_case_globals, missing_docs)]
        pub const #iid_ident: vst3_com::sys::IID = vst3_com::sys::IID {
            data: [#(#bytes),*]
        };
    )
}

pub fn ident(interface_ident: &Ident) -> Ident {
    format_ident!(
        "IID_{}",
        crate::utils::camel_to_snake(&interface_ident.to_string()).to_uppercase()
    )
}
