use quote::quote;
use syn::Field;

use mokuya::components::prelude::*;

pub fn generate_sum(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let method_name_sum = new_ident("sum", field_name);

    quote! {
        pub fn #method_name_sum(&mut self, other: #field_type) {
            self.#field_name += other;
        }
    }
}
