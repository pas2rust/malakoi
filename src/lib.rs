use mokuya::components::prelude::*;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod components;

#[proc_macro_derive(Math)]
pub fn math(input: TokenStream) -> TokenStream {
    use components::component::generate_math;

    let mut input = parse_macro_input!(input as DeriveInput);
    add_traits_to_generics(&mut input);

    let generate_math = generate_math(&input);
    generate_math.into()
}
