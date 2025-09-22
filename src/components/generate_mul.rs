use quote::quote;
use syn::Field;

use mokuya::components::prelude::*;

pub fn generate_mul(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let method_name_mul = new_ident("mul", field_name);

    quote! {
        pub fn #method_name_mul(&mut self, other: #field_type) {
            self.#field_name *= other;
        }
    }
}
