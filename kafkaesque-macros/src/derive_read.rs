use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Type, Variant};

#[derive(FromDeriveInput, Debug)]
#[darling(supports(struct_named, struct_newtype, struct_unit))]
struct Params {
    ident: syn::Ident,
    data: darling::ast::Data<Variant, StructField>,
}

#[derive(Debug, Clone, FromField)]
struct StructField {
    ident: Option<Ident>,
    ty: Type,
}

pub fn expand(ts: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(ts as DeriveInput);
    let params = Params::from_derive_input(&derive_input).expect("Failed to parse inputs");

    let name = params.ident;
    let fields = match params.data {
        Data::Struct(fields) => fields,
        Data::Enum(_) => todo!(""),
    };

    let reading: Vec<proc_macro2::TokenStream> = fields
        .into_iter()
        .map(|field| {
            let name = field
                .ident
                .clone()
                .map(|ident| quote! {#ident})
                .unwrap_or(quote! {0});
            (name, field.ty)
        })
        .map(|(name, ty)| {
            quote! { #name: <#ty as Read>::read_from(reader).await?, }
        })
        .collect();

    let output = quote! {
        impl crate::protocol::codec::Read for #name {
            async fn read_from(reader: &mut (dyn tokio::io::AsyncRead + Send + Unpin)) -> Result<Self> {
                let v = #name {
                    #(#reading) *
                };
                Ok(v)
            }
        }
    };

    output.into()
}
