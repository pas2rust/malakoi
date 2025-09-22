use quote::quote;
use syn::Field;

use mokuya::components::prelude::*;

pub fn generate_discount(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let method_name_discount = new_ident("discount", field_name);

    quote! {
        pub fn #method_name_discount(&mut self, percentage: #field_type) {
            self.#field_name = {
                let value = self.#field_name as f64;
                (value - (value * (percentage as f64 / 100.0))) as #field_type
            };
        }
    }
}
