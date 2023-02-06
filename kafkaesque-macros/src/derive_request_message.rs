use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(request_message))]
#[darling(supports(struct_any))]
struct Params {
    ident: syn::Ident,
    version: i16,
    key: Ident,
}

pub fn expand(ts: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(ts as DeriveInput);
    let params = Params::from_derive_input(&derive_input).expect("Failed to parse inputs");

    let name = params.ident;
    let version = params.version;
    let key = params.key;

    let output = quote! {
        #[automatically_derived]
        impl crate::protocol::request::RequestMessage for #name {
            const API_KEY: ApiKey = crate::protocol::api_keys::ApiKey::#key;
            const API_VERSION: ApiVersion = crate::protocol::request::ApiVersion(#version);
        }
    };

    output.into()
}
