use proc_macro::{self, TokenStream};
use proc_macro2::{Punct, Span};
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, FieldsNamed, PathSegment, Token};

struct ConstructorArg {
    ident: syn::Ident,
    r#type: syn::Type,
}
impl quote::ToTokens for ConstructorArg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.ident.to_tokens(tokens);
        Punct::new(':', proc_macro2::Spacing::Alone).to_tokens(tokens);
        self.r#type.to_tokens(tokens);
    }
}

struct ConstructorOptArg {
    ident: syn::Ident,
}
impl quote::ToTokens for ConstructorOptArg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.ident.to_tokens(tokens);
        Punct::new(':', proc_macro2::Spacing::Alone).to_tokens(tokens);

        let mut p: Punctuated<PathSegment, syn::token::PathSep> = Punctuated::new();
        p.push(PathSegment::from(syn::Ident::new("std", Span::call_site())));
        p.push_punct(Token![::](Span::call_site()));
        p.push(PathSegment::from(syn::Ident::new("option", Span::call_site())));
        p.push_punct(Token![::](Span::call_site()));
        p.push(PathSegment::from(syn::Ident::new("Option", Span::call_site())));
        p.push_punct(Token![::](Span::call_site()));
        p.push(PathSegment::from(syn::Ident::new("None", Span::call_site())));
        p.to_tokens(tokens);
    }
}

#[proc_macro_derive(ApiRequestToString)]
pub fn api_request_to_string(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input as DeriveInput);

    quote! {
        impl ::std::string::ToString for #ident{
            fn to_string(&self) ->String{
                ::serde_qs::to_string(self).unwrap()
            }
        }
    }
    .into()
}

fn api_request_require_parse_struct_named_field(
    fields: FieldsNamed,
) -> (Vec<ConstructorArg>, Vec<ConstructorOptArg>, bool) {
    let mut cons_args = vec![];
    let mut opt_args = vec![];
    let mut has_timestamp = false;
    for named in fields.named.into_iter() {
        match &named.ty {
            syn::Type::Path(type_path) => {
                if type_path.path.segments.first().unwrap().ident == "Option" {
                    opt_args.push(ConstructorOptArg { ident: named.ident.unwrap() });
                } else if named.ident.as_ref().unwrap() == "timestamp" {
                    has_timestamp = true;
                    continue;
                } else {
                    let arg = ConstructorArg { ident: named.ident.unwrap(), r#type: named.ty };
                    cons_args.push(arg);
                }
            }
            _ => {
                if named.ident.as_ref().unwrap() == "timestamp" {
                    has_timestamp = true;
                    continue;
                } else {
                    cons_args.push(ConstructorArg { ident: named.ident.unwrap(), r#type: named.ty });
                }
            }
        }
    }
    (cons_args, opt_args, has_timestamp)
}

#[proc_macro_derive(ApiRequestRequire)]
pub fn api_request_require_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    match data {
        syn::Data::Struct(stct) => match stct.fields {
            syn::Fields::Named(named_fields) => {
                let (cons_args, opt_args, has_timestamp) = api_request_require_parse_struct_named_field(named_fields);

                let required_field_names = cons_args.iter().fold(vec![], |mut acc, v| {
                    acc.push(v.ident.clone());
                    acc
                });
                let init_timestamp = if has_timestamp {
                    Some(quote! {
                    timestamp: std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64,
                            })
                } else {
                    None
                };

                let comma_after_required = if !required_field_names.is_empty() {
                    Some(Token![,](Span::call_site()))
                } else {
                    None
                };
                let comma_after_opts = if !opt_args.is_empty() {
                    Some(Token![,](Span::call_site()))
                } else {
                    None
                };

                let output = quote! {
                    impl #ident {
                        pub fn require(#(#cons_args),*) -> Self {
                            #ident {
                                #(#required_field_names),*
                                #comma_after_required
                                #(#opt_args),*
                                #comma_after_opts
                                #init_timestamp
                            }
                        }
                    }
                };

                output.into()
            }
            syn::Fields::Unnamed(_) => {
                todo!();
                // let mut output: Option<TokenStream> = None;

                // if unnamed.unnamed.len() != 1 {
                //     unimplemented!()
                // }

                // let the_field = unnamed.unnamed.first().unwrap();
                //     match the_field.ty {
                //         syn::Type::Path(type_path) => {
                //             if type_path.path.get_ident().unwrap() == "BaseRequest" {
                //                 // do stuff

                //                 type_path
                //             }
                //         }
                //         _ => continue,
                //     }

                // if let Some(o) = output {
                //     return o.into();
                // } else {
                //     return TokenStream::new().into();
                // }
            }
            syn::Fields::Unit => todo!(),
        },
        syn::Data::Enum(data_enum) => {
            for variant in data_enum.variants.iter() {
                println!("{:?}", variant.attrs)
            }
            unimplemented!()
        }
        syn::Data::Union(_) => {
            unimplemented!()
        }
    }
}
