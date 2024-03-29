use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse_macro_input, DeriveInput, Ident, Type, Variant};

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
    ty: Type,
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
                .push(parse(quote! { Read }.into()).unwrap());
        }
        g
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
        .map(|(f_name, ty)| {
            quote! { #f_name: {
                tracing::trace!("reading {}", stringify!(#name.#f_name));
                <#ty as Read>::read_from(reader).await? }
            }
        })
        .collect();

    let output = quote! {
        #[automatically_derived]
        impl #impl_generics crate::formats::codec::Read for #name #generics {
            async fn read_from(reader: &mut (dyn tokio::io::AsyncRead + Send + Unpin)) -> crate::formats::Result<Self> {
                let v = #name {
                    #(#reading),*
                };
                Ok(v)
            }
        }
    };

    output.into()
}
