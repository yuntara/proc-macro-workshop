use syn::DeriveInput;

mod builder;
mod util;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    builder::expand_derive_builder(&input)
}
