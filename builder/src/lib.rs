use darling::{ast, FromDeriveInput, FromField, FromMeta};
use quote::{format_ident, quote, ToTokens};
use syn::{DeriveInput, GenericArgument, Path, PathArguments, PathSegment, Type, TypePath};

#[derive(Debug, FromField)]
#[darling(attributes(builder))]
struct BuilderFieldReceiver {
    ident: Option<syn::Ident>,

    ty: syn::Type,
    each: Option<String>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(builder), supports(struct_any))]
struct BuilderInputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<(), BuilderFieldReceiver>,
}

impl ToTokens for BuilderInputReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let BuilderInputReceiver {
            ref ident,
            ref generics,
            ref data,
        } = *self;

        // let (imp, ty, wher) = generics.split_for_impl();

        let struct_name = ident;
        let builder_name = format_ident!("{}Builder", ident);

        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        let wrapped_fields_stream_iter = fields.iter().map(|field| {
            let ty = &field.ty;
            let ident = &field.ident;
            if is_option_type(&ty) {
                quote! {
                    #ident: #ty
                }
            } else {
                quote! {
                    #ident: Option<#ty>
                }
            }
        });

        let initial_fileds_stream_iter = fields.iter().map(|field| {
            let ty = &field.ty;
            let ident = &field.ident;
            let each = &field.each;

            if is_vec_type(&ty) && each.is_some() {
                quote! {
                    #ident: Some(vec![])
                }
            } else {
                quote! {
                    #ident: None
                }
            }
        });

        let builder_fields_setter_stream_iter = fields.iter().map(|field| {
            let ty = &field.ty;
            let ident = &field.ident;
            let each = &field.each;

            if is_vec_type(&ty) && each.is_some() {
                let inner_type = extract_inner_type(&ty);
                let lit = each.as_ref().unwrap();
                let lit_ident = format_ident!("{}", lit);

                if *lit == ident.clone().unwrap().to_string() {
                    let ref_ident = format_ident!("ref_{}", lit);
                    quote! {
                        fn #ident(&mut self, #lit_ident: #inner_type) -> &mut Self {
                            if let Some(ref mut #ref_ident) = self.#ident {
                                #ref_ident.push(#lit_ident);
                            } else {
                                self.#ident = Some(vec![#lit_ident]);
                            };
                            self
                        }
                    }
                } else {
                    quote! {
                        fn #lit_ident(&mut self, #lit_ident: #inner_type) -> &mut Self {
                            if let Some(ref mut #ident) = self.#ident {
                                #ident.push(#lit_ident);
                            } else {
                                self.#ident = Some(vec![#lit_ident]);
                            };
                            self
                        }

                        fn #ident(&mut self, #ident: #ty) -> &mut Self {
                            self.#ident = Some(#ident);
                            self
                        }
                    }
                }
            } else if is_option_type(&ty) {
                let inner_type = extract_inner_type(&ty);
                quote! {
                    fn #ident(&mut self, #ident: #inner_type) -> &mut Self {
                        self.#ident = Some(#ident);
                        self
                    }
                }
            } else {
                quote! {
                    fn #ident(&mut self, #ident: #ty) -> &mut Self {
                        self.#ident = Some(#ident);
                        self
                    }
                }
            }
        });
        let builder_build_stream_iter = fields.iter().map(|field| {
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
        });
        tokens.extend(quote! {
            pub struct #builder_name {
                #(#wrapped_fields_stream_iter),*
            }
            impl #struct_name {
                pub fn builder() -> #builder_name {
                    #builder_name {
                        #(#initial_fileds_stream_iter),*
                    }
                }
            }

            impl #builder_name {
                #(#builder_fields_setter_stream_iter)*

                pub fn build(&mut self) -> Result<#struct_name, Box<dyn std::error::Error>> {
                    Ok(#struct_name {
                        #(#builder_build_stream_iter),*
                    })
                }
            }

        });
    }
}

fn is_option_type(ty: &Type) -> bool {
    match last_path_segment(&ty) {
        Some(path_seg) => path_seg.ident == "Option",
        None => false,
    }
}

fn is_vec_type(ty: &Type) -> bool {
    match last_path_segment(&ty) {
        Some(path_seg) => path_seg.ident == "Vec",
        None => false,
    }
}

fn extract_inner_type(ty: &Type) -> &GenericArgument {
    match last_path_segment(&ty) {
        Some(PathSegment {
            ident: _,
            arguments: PathArguments::AngleBracketed(ref gen_arg),
        }) => gen_arg.args.first(),
        _ => None,
    }
    .expect("invalid option type")
}

fn last_path_segment(ty: &Type) -> Option<&PathSegment> {
    match ty {
        &Type::Path(TypePath {
            qself: None,
            path:
                Path {
                    segments: ref seg,
                    leading_colon: _,
                },
        }) => seg.last(),
        _ => None,
    }
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    expand_derive_builder(&input)
}
fn expand_derive_builder(input: &syn::DeriveInput) -> proc_macro::TokenStream {
    let builder = match BuilderInputReceiver::from_derive_input(&input) {
        Ok(parsed) => parsed,
        Err(e) => return e.write_errors().into(),
    };
    quote! {
        #builder
    }
    .into()
}
