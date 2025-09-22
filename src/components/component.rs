use mokuya::components::prelude::*;
use quote::quote;
use syn::DeriveInput;

use crate::components::{
    generate_approach::generate_approach, generate_discount::generate_discount,
    generate_div::generate_div, generate_inflate::generate_inflate, generate_mul::generate_mul,
    generate_sub::generate_sub, generate_sum::generate_sum,
};

pub fn generate_math(input: &DeriveInput) -> proc_macro2::TokenStream {
    let impl_block = get_impl(input);
    let mut methods: Vec<proc_macro2::TokenStream> = Vec::new();

    for field in get_named_fields(input).unwrap().named.iter() {
        methods.push(generate_discount(field));
        methods.push(generate_div(field));
        methods.push(generate_mul(field));
        methods.push(generate_sub(field));
        methods.push(generate_sum(field));
        methods.push(generate_inflate(field));
        methods.push(generate_approach(field));
    }

    quote! {
        //#[cfg_attr(feature = "tracing", mdd::debugger_impl)]
        impl #impl_block {
            #(#methods)*
        }
    }
}
