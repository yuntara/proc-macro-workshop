use syn::DeriveInput;

mod builder;
mod util;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    match builder::expand_derive_builder(&input) {
        Ok(tokens) => tokens,
        Err(e) => e.into_compile_error().into(),
    }
}
