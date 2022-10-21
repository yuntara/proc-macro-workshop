use darling::{ast, FromDeriveInput, FromField, FromMeta};
use quote::{format_ident, quote, ToTokens};

use syn::{Error, Result};

use super::util::*;

#[derive(Debug, FromField)]
#[darling(attributes(builder))]
pub(crate) struct BuilderFieldReceiver {
    ident: Option<syn::Ident>,

    ty: syn::Type,
    each: Option<String>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(builder), supports(struct_any))]
pub(crate) struct BuilderInputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<(), BuilderFieldReceiver>,
}

struct RenderFields {
    builder_name: syn::Ident,
    wrapped_fields: Vec<proc_macro2::TokenStream>,
    struct_name: syn::Ident,
    initial_fileds: Vec<proc_macro2::TokenStream>,
    builder_fields_setter: Vec<proc_macro2::TokenStream>,
    builder_build: Vec<proc_macro2::TokenStream>,
}
fn determine_fields(
    BuilderInputReceiver {
        ident,
        generics,
        data,
    }: BuilderInputReceiver,
) -> Result<RenderFields> {
    let builder_name = format_ident!("{}Builder", ident);
    let struct_name = ident.clone();

    let fields = data
        .as_ref()
        .take_struct()
        .ok_or_else(|| Error::new(ident.span(), "must be Struct"))?
        .fields;

    let wrapped_fields = fields
        .iter()
        .map(|field| {
            let ty = &field.ty;
            let ident = &field.ident;
            if is_option_type(&ty) {
                quote! {
                    #ident: #ty
                }
            } else {
                quote! {
                    #ident: std::option::Option<#ty>
                }
            }
        })
        .collect();

    let initial_fileds = fields
        .iter()
        .map(|field| {
            let ty = &field.ty;
            let ident = &field.ident;
            let each = &field.each;

            if is_vec_type(ty) && each.is_some() {
                quote! {
                    #ident: std::option::Option::Some(vec![])
                }
            } else {
                quote! {
                    #ident: std::option::Option::None
                }
            }
        })
        .collect();

    let builder_fields_setter = fields.iter().map(|field| {
        let BuilderFieldReceiver {
            ty,
            ident: field_ident,
            each,
        } = field;

        if is_vec_type(ty) && field.each.is_some() {
            let inner_type = extract_inner_type(ty);
            let each_method_name = each.as_ref().unwrap();
            let each_method_name_ident = format_ident!("{}", each_method_name);

            if &each_method_name_ident == field_ident.as_ref().unwrap() {
                let ref_ident = format_ident!("ref_{}", each_method_name);
                quote! {
                    fn #field_ident(&mut self, #each_method_name_ident: #inner_type) -> &mut Self {
                        if let std::option::Option::Some(ref mut #ref_ident) = self.#field_ident {
                            #ref_ident.push(#each_method_name_ident);
                        } else {
                            self.#field_ident = std::option::Option::Some(vec![#each_method_name_ident]);
                        };
                        self
                    }
                }
            } else {
                quote! {
                    fn #each_method_name_ident(&mut self, #each_method_name_ident: #inner_type) -> &mut Self {
                        if let std::option::Option::Some(ref mut #field_ident) = self.#field_ident {
                            #field_ident.push(#each_method_name_ident);
                        } else {
                            self.#field_ident = std::option::Option::Some(vec![#each_method_name_ident]);
                        };
                        self
                    }

                    fn #field_ident(&mut self, #field_ident: #ty) -> &mut Self {
                        self.#field_ident = std::option::Option::Some(#field_ident);
                        self
                    }
                }
            }
        } else if is_option_type(&ty) {
            let inner_type = extract_inner_type(&ty);
            quote! {
                fn #field_ident(&mut self, #field_ident: #inner_type) -> &mut Self {
                    self.#field_ident = std::option::Option::Some(#field_ident);
                    self
                }
            }
        } else {
            quote! {
                fn #field_ident(&mut self, #field_ident: #ty) -> &mut Self {
                    self.#field_ident = std::option::Option::Some(#field_ident);
                    self
                }
            }
        }
    }).collect();
    let builder_build = fields
        .iter()
        .map(|field| {
            let ty = &field.ty;
            let ident = &field.ident;

            if is_option_type(&ty) {
                quote! {
                    #ident: self.#ident.clone()
                }
            } else {
                quote! {
                    #ident: self.#ident.clone().unwrap()
                }
            }
        })
        .collect();
    Ok(RenderFields {
        builder_name,
        wrapped_fields,
        struct_name,
        initial_fileds,
        builder_fields_setter,
        builder_build,
    })
}
fn render_builder(
    RenderFields {
        builder_name,
        wrapped_fields,
        struct_name,
        initial_fileds,
        builder_fields_setter,
        builder_build,
    }: RenderFields,
) -> proc_macro2::TokenStream {
    quote! {
        pub struct #builder_name {
            #(#wrapped_fields),*
        }
        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#initial_fileds),*
                }
            }
        }

        impl #builder_name {
            #(#builder_fields_setter)*

            pub fn build(&mut self) -> std::result::Result<#struct_name, std::boxed::Box<dyn std::error::Error>> {
                std::result::Result::Ok(#struct_name {
                    #(#builder_build),*
                })
            }
        }

    }
}
pub(crate) fn expand_derive_builder(input: &syn::DeriveInput) -> Result<proc_macro::TokenStream> {
    let builder = match BuilderInputReceiver::from_derive_input(input) {
        Ok(parsed) => parsed,
        Err(e) => return Err(e.into()),
    };
    let fields = determine_fields(builder)?;

    Ok(render_builder(fields).into())
}
