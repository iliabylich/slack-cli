extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod http_resource_macro;
use http_resource_macro::macro_impl;

#[proc_macro_derive(HttpResource, attributes(result))]
pub fn resource_macro(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);

    match macro_impl(input) {
        Ok(output) => TokenStream::from(output),
        Err(err) => {
            let macro_error = format!("#[HttpResource] {}", err);
            let quoted_error = quote! {
                compile_error!(#macro_error);
            };
            quoted_error.into()
        }
    }
}

