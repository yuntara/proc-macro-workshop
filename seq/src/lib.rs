use std::io::Write;

use darling::ToTokens;
use proc_macro::TokenStream;
use quote::format_ident;

struct Seq {
    name: syn::Ident,
    from: isize,
    to: isize,
    brace_token: syn::token::Brace,
    tokens: proc_macro2::TokenStream,
}

impl syn::parse::Parse for Seq {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let name: syn::Ident = input.parse()?;

        input.parse::<syn::Token![in]>()?;

        let from: syn::LitInt = input.parse()?;

        input.parse::<syn::Token![..]>()?;

        let to: syn::LitInt = input.parse()?;

        let content;

        let brace_token = syn::braced!(content in input);

        let tokens: proc_macro2::TokenStream = content.parse()?;

        let from = from.base10_parse::<isize>()?;
        let to = to.base10_parse::<isize>()?;

        Ok(Seq {
            name,
            from,
            to,
            brace_token,
            tokens,
        })
    }
}

fn replace_var(
    tokens: proc_macro2::TokenStream,
    name: syn::Ident,
    value: isize,
) -> proc_macro2::TokenStream {
    let tokens: Vec<_> = tokens.into_iter().collect();
    let mut token_streams: Vec<_> = vec![];
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens.get(i);
        if token.is_none() {
            break;
        }
        let token = token.unwrap();

        let tokens: proc_macro2::TokenStream = match token {
            proc_macro2::TokenTree::Group(group) => {
                proc_macro2::TokenTree::Group(proc_macro2::Group::new(
                    group.delimiter(),
                    replace_var(group.stream(), name.clone(), value),
                ))
                .to_token_stream()
            }
            proc_macro2::TokenTree::Ident(ident) => {
                if ident == &name {
                    if i < 2 {
                        proc_macro2::TokenTree::Literal(proc_macro2::Literal::isize_unsuffixed(
                            value,
                        ))
                        .into_token_stream()
                    } else if let Some(proc_macro2::TokenTree::Punct(punct)) = tokens.get(i - 1) {
                        if punct.as_char() == '~' {
                            token_streams.pop(); // remove ~
                            token_streams.pop();
                            let last_ident = tokens.get(i - 2);
                            if let Some(proc_macro2::TokenTree::Ident(last_ident)) = last_ident {
                                let ident = format_ident!("{}{}", last_ident, value as usize);
                                ident.into_token_stream()
                            } else {
                                quote::quote! {}
                            }
                        } else {
                            proc_macro2::TokenTree::Literal(proc_macro2::Literal::isize_unsuffixed(
                                value,
                            ))
                            .into_token_stream()
                        }
                    } else {
                        proc_macro2::TokenTree::Literal(proc_macro2::Literal::isize_unsuffixed(
                            value,
                        ))
                        .into_token_stream()
                    }
                } else {
                    proc_macro2::TokenTree::Ident(ident.clone()).into_token_stream()
                }
            }
            proc_macro2::TokenTree::Literal(lit) => {
                proc_macro2::TokenTree::Literal(lit.clone()).into_token_stream()
            }
            proc_macro2::TokenTree::Punct(punct) => {
                proc_macro2::TokenTree::Punct(punct.clone()).into_token_stream()
            }
        };
        token_streams.push(tokens);

        i += 1;
    }
    quote::quote! { #(#token_streams)* }
}

fn repeat_section(
    tokens: proc_macro2::TokenStream,
    name: syn::Ident,
    from: isize,
    to: isize,
) -> (proc_macro2::TokenStream, bool) {
    let tokens: Vec<_> = tokens.into_iter().collect();
    let mut token_streams: Vec<_> = vec![];
    let mut i = 0;
    let mut has_repeat_section = false;
    while i < tokens.len() {
        let token = &tokens.get(i);
        if token.is_none() {
            break;
        }
        let token = token.unwrap();

        let tokens: proc_macro2::TokenStream = match token {
            proc_macro2::TokenTree::Group(group) => {
                let (t, has_r) = repeat_section(group.stream(), name.clone(), from, to);
                has_repeat_section = has_repeat_section || has_r;
                proc_macro2::TokenTree::Group(proc_macro2::Group::new(group.delimiter(), t))
                    .to_token_stream()
            }
            proc_macro2::TokenTree::Ident(ident) => {
                proc_macro2::TokenTree::Ident(ident.clone()).into_token_stream()
            }
            proc_macro2::TokenTree::Literal(lit) => {
                proc_macro2::TokenTree::Literal(lit.clone()).into_token_stream()
            }
            proc_macro2::TokenTree::Punct(punct) => {
                if punct.as_char() == '#' {
                    if let Some(proc_macro2::TokenTree::Group(group)) = tokens.get(i + 1) {
                        if let Some(proc_macro2::TokenTree::Punct(punct)) = tokens.get(i + 2) {
                            if punct.as_char() == '*'
                                && group.delimiter() == proc_macro2::Delimiter::Parenthesis
                            {
                                let mut t = vec![];
                                for i in from..to {
                                    t.push(replace_var(group.stream(), name.clone(), i));
                                }
                                has_repeat_section = true;
                                token_streams.push(quote::quote! { #(#t)* });
                                i += 3;
                                continue;
                            }
                        }
                    }
                }
                proc_macro2::TokenTree::Punct(punct.clone()).into_token_stream()
            }
        };
        token_streams.push(tokens);

        i += 1;
    }
    (quote::quote! { #(#token_streams)* }, has_repeat_section)
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let Seq {
        name,
        from,
        to,
        tokens,
        ..
    } = syn::parse_macro_input!(input as Seq);

    let mut t: Vec<proc_macro2::TokenStream> = vec![];

    let (tokens, has_repeat) = repeat_section(tokens, name.clone(), from, to);

    if !has_repeat {
        for i in from..to {
            t.push(replace_var(tokens.clone(), name.clone(), i));
        }

        let expanded = quote::quote! { #(#t)* };
        TokenStream::from(expanded)
    } else {
        let expanded = quote::quote! { #tokens };
        TokenStream::from(expanded)
    }
}
