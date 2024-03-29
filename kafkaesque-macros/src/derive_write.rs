use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use std::iter;
use syn::{parse, parse_macro_input, DeriveInput, Ident, Variant};

#[derive(FromDeriveInput, Debug)]
#[darling(supports(struct_named, struct_newtype, struct_unit))]
struct Params {
    ident: syn::Ident,
    data: darling::ast::Data<Variant, StructField>,
    generics: syn::Generics,
}

#[derive(Debug, Clone, FromField)]
struct StructField {
    ident: Option<Ident>,
    // ty: Type,
}

pub fn expand(ts: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(ts as DeriveInput);
    let params = Params::from_derive_input(&derive_input).expect("Failed to parse inputs");

    let generics = params.generics;
    let name = params.ident;
    let fields = match params.data {
        Data::Struct(fields) => fields,
        Data::Enum(_) => unimplemented!("Unsupported"),
    };

    let impl_generics = {
        let mut g = generics.clone();
        for type_param in g.type_params_mut() {
            type_param
                .bounds
                .push(parse(quote! { Write }.into()).unwrap());
        }
        g
    };

    let field_names: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            field
                .ident
                .clone()
                .map(|ident| quote! {#ident})
                .unwrap_or(quote! {0})
        })
        .collect();

    let size_calculation: Vec<proc_macro2::TokenStream> = field_names
        .iter()
        .map(|name| {
            quote! { Write::calculate_size(&self.#name) }
        })
        .chain(iter::once(quote! { 0 }))
        .collect();

    let writing: Vec<proc_macro2::TokenStream> = field_names
        .iter()
        .map(|name| {
            quote! { Write::write_to(&self.#name, writer).await?;}
        })
        .collect();

    let output = quote! {
        #[automatically_derived]
        impl #impl_generics crate::formats::codec::Write for #name #generics {
            fn calculate_size(&self) -> i32 {
                #(#size_calculation)+*
            }
            async fn write_to(&self, writer: &mut (dyn tokio::io::AsyncWrite + Send + Unpin)) -> crate::formats::Result<()> {
                #(#writing) *
                Ok(())
            }
        }
    };

    output.into()
}
