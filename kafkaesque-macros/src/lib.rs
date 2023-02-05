mod derive_write;
mod derive_read;

use proc_macro::TokenStream;

#[proc_macro_derive(Write)]
pub fn derive_write(input: TokenStream) -> TokenStream {
    derive_write::expand(input)
}

#[proc_macro_derive(Read)]
pub fn derive_read(input: TokenStream) -> TokenStream {
    derive_read::expand(input)
}
