use quote::quote;
use syn::Field;

use mokuya::components::prelude::*;

pub fn generate_div(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let method_name_div = new_ident("div", field_name);

    quote! {
        pub fn #method_name_div(&mut self, other: #field_type) {
            self.#field_name /= other;
        }
    }
}
