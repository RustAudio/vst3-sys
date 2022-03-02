use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as HelperTokenStream};
use quote::{format_ident, quote};
use syn::LitInt;

pub fn generate(macro_attr: &TokenStream, interface_ident: &Ident) -> HelperTokenStream {
    let iid_string: syn::LitStr =
        syn::parse(macro_attr.clone()).expect("[com_interface] parameter must be a GUID string");
    let iid_value = iid_string.value();

    assert_eq!(
        iid_value.len(),
        36,
        "IIDs must be exactly 36 characters long"
    );

    let iid_ident = ident(interface_ident);
    let iid_value = iid_value.as_str();
    let delimited: Vec<&str> = iid_value.split('-').collect();

    assert_eq!(
        delimited.len(),
        5,
        "IIDs must have 5 parts separate by '-'s"
    );

    assert_eq!(
        delimited[0].len(),
        8,
        "The first part of the IID must be 8 characters long, but it is {} characters long",
        delimited[0].len()
    );

    assert_eq!(
        delimited[1].len(),
        4,
        "The second part of the IID must be 4 characters long, but it is {} characters long",
        delimited[1].len()
    );

    assert_eq!(
        delimited[2].len(),
        4,
        "The third part of the IID must be 4 characters long, but it is {} characters long",
        delimited[2].len()
    );

    assert_eq!(
        delimited[3].len(),
        4,
        "The fourth part of the IID must be 4 characters long, but it is {} characters long",
        delimited[3].len()
    );

    assert_eq!(
        delimited[4].len(),
        12,
        "The fifth part of the IID must be 12 characters long, but it is {} characters long",
        delimited[4].len()
    );

    let flat = delimited
        .into_iter()
        .flat_map(|s| s.chars())
        .collect::<Vec<_>>();

    // The byte order is different on Windows compared to the other operating systems. Because this
    // is a proc macro, `#[cfg(...)]` queries the _host's/compiler's_ environment, and not the
    // actual target environment. Because of that cross compiling won't work if we try to do the
    // byte swap here. To get around that, we'll generate both versions, and then emit the
    // #[cfg(...)] attributes in the generated code.
    let mut flat_windows = flat.clone();
    flat_windows.swap(0, 6);
    flat_windows.swap(1, 7);
    flat_windows.swap(2, 4);
    flat_windows.swap(3, 5);
    flat_windows.swap(8, 10);
    flat_windows.swap(9, 11);
    flat_windows.swap(12, 14);
    flat_windows.swap(13, 15);

    let to_hex_int_array = |char_slice: Vec<char>| {
        (0..32).step_by(2).map(move |idx| {
            let mut chars = ['0', 'x', '\0', '\0'];
            chars[2] = char_slice[idx];
            chars[3] = char_slice[idx + 1];
            let string = chars.iter().collect::<String>();
            LitInt::new(&string, Span::call_site())
        })
    };
    let bytes = to_hex_int_array(flat);
    let bytes_windows = to_hex_int_array(flat_windows);

    quote!(
        #[cfg(not(target_os = "windows"))]
        #[allow(non_upper_case_globals, missing_docs)]
        pub const #iid_ident: vst3_com::sys::IID = vst3_com::sys::IID {
            data: [#(#bytes),*]
        };

        #[cfg(target_os = "windows")]
        #[allow(non_upper_case_globals, missing_docs)]
        pub const #iid_ident: vst3_com::sys::IID = vst3_com::sys::IID {
            data: [#(#bytes_windows),*]
        };
    )
}

pub fn ident(interface_ident: &Ident) -> Ident {
    format_ident!(
        "IID_{}",
        crate::utils::camel_to_snake(&interface_ident.to_string()).to_uppercase()
    )
}
