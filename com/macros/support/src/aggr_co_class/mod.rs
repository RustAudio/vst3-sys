extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{AttributeArgs, ItemStruct};

use std::iter::FromIterator;

mod class_factory;
mod com_struct;
mod com_struct_impl;
mod iunknown_impl;

pub fn expand_aggr_co_class(input: &ItemStruct, attr_args: &AttributeArgs) -> TokenStream {
    let base_interface_idents = crate::utils::base_interface_idents(attr_args);
    let aggr_interface_idents = crate::utils::get_aggr_map(attr_args);
    let out: Vec<TokenStream> = vec![
        com_struct::generate(&aggr_interface_idents, &base_interface_idents, input).into(),
        com_struct_impl::generate(&base_interface_idents, &aggr_interface_idents, input).into(),
        crate::co_class::co_class_impl::generate(input).into(),
        iunknown_impl::generate(input).into(),
        class_factory::generate(input).into(),
    ];
    TokenStream::from_iter(out)
}
