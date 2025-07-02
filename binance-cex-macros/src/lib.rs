use proc_macro::{self, TokenStream};
use proc_macro2::{Punct, Span};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DataEnum, DataStruct, DeriveInput,
    FieldsNamed, Ident, Meta, Path, PathSegment, Token,
};

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
        p.push(PathSegment::from(syn::Ident::new(
            "option",
            Span::call_site(),
        )));
        p.push_punct(Token![::](Span::call_site()));
        p.push(PathSegment::from(syn::Ident::new(
            "Option",
            Span::call_site(),
        )));
        p.push_punct(Token![::](Span::call_site()));
        p.push(PathSegment::from(syn::Ident::new(
            "None",
            Span::call_site(),
        )));
        p.to_tokens(tokens);
    }
}

#[proc_macro_derive(APIEndPoint, attributes(endpoint))]
pub fn api_end_point(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    if let Data::Enum(DataEnum { variants, .. }) = data {
        let mut methods = vec![];
        let mut security_types = vec![];
        let mut variant_idents = vec![];
        let mut urls = vec![];
        for v in variants.iter() {
            let mut method = None;
            let mut security_type = None;
            let mut url = None;
            let mut dup = false;
            for attr in v.attrs.iter() {
                if !attr.path().is_ident("endpoint") {
                    continue;
                }
                if dup {
                    panic!("there must be only one endpoint attr");
                }
                dup = true;

                let nested = attr
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .unwrap();
                for meta in nested.iter() {
                    match meta {
                        Meta::Path(path) => {
                            let path_ident = path.get_ident().unwrap();
                            match path_ident.to_string().as_str() {
                                st @ "None"
                                | st @ "UserData"
                                | st @ "UserStream"
                                | st @ "MarketData"
                                | st @ "Margin"
                                | st @ "Trade" => {
                                    assert!(security_type.is_none());
                                    let mut security_type_path =
                                        Path::from(Ident::new("super", Span::call_site()));
                                    security_type_path.segments.push_punct(Token![::](Span::call_site()));
                                    security_type_path.segments.push_value(PathSegment::from(
                                        Ident::new("SecurityType", Span::call_site()),
                                    ));
                                    security_type_path.segments.push_punct(Token![::](Span::call_site()));
                                    security_type_path.segments.push_value(PathSegment::from(
                                        Ident::new(st, Span::call_site()),
                                    ));
                                    security_type.replace(security_type_path);
                                }
                                m @ "GET" | m @ "POST" | m @ "PUT" | m @ "DELETE" => {
                                    assert!(method.is_none());
                                    let mut http_method_path =
                                        Path::from(Ident::new("http", Span::call_site()));
                                    http_method_path
                                        .leading_colon
                                        .replace(Token![::](Span::call_site()));
                                    http_method_path.segments.push_punct(Token![::](Span::call_site()));
                                    http_method_path.segments.push_value(PathSegment::from(
                                        Ident::new("Method", Span::call_site()),
                                    ));
                                    http_method_path.segments.push_punct(Token![::](Span::call_site()));
                                    http_method_path.segments.push_value(PathSegment::from(
                                        Ident::new(m, Span::call_site()),
                                    ));
                                    method.replace(http_method_path);
                                }
                                _ => {
                                    panic!("unknown endpoint attr")
                                }
                            }
                        }
                        Meta::List(_meta_list) => todo!(),
                        Meta::NameValue(meta_name_value) => {
                            if !meta_name_value.path.is_ident("url") {
                                panic!("unknown endpoint attr");
                            }
                            assert!(url.is_none());
                            url.replace(meta_name_value.value.clone());
                        }
                    }
                }
            }

            if url.is_none() {
                panic!("missing url");
            }
            if method.is_none() {
                panic!("method http method");
            }
            if security_type.is_none() {
                panic!("missing security type");
            }

            variant_idents.push(v.ident.clone());
            methods.push(method.unwrap());
            security_types.push(security_type.unwrap());
            urls.push(url.unwrap());
        }

        quote! {
            impl Endpoint for #ident {
                fn action_params(&self) -> (http::Method, SecurityType, String) {
                    match self {
                        #(#ident::#variant_idents => (#methods, #security_types, ::std::string::String::from(#urls)),)*
                    }
                }
            }
        }
        .into()
    } else {
        unreachable!("only enums are supported");
    }
}

#[proc_macro_derive(APIRequestToString)]
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

fn api_request_init_parse_struct_named_field(
    fields: FieldsNamed,
) -> (Vec<ConstructorArg>, Vec<ConstructorOptArg>, bool) {
    let std_option = Punctuated::from_iter([
        PathSegment::from(Ident::new("std", Span::call_site())),
        PathSegment::from(Ident::new("option", Span::call_site())),
        PathSegment::from(Ident::new("Option", Span::call_site())),
    ]);
    let mut cons_args = vec![];
    let mut opt_args = vec![];
    let mut has_timestamp = false;
    for named_field in fields.named.into_iter() {
        match &named_field.ty {
            syn::Type::Path(type_path) => {
                if type_path.path.leading_colon.is_some() {
                    if type_path.path.segments.eq(&std_option) {
                        opt_args.push(ConstructorOptArg {
                            ident: named_field.ident.unwrap(),
                        });
                        continue;
                    }
                }

                if type_path.path.segments.first().unwrap().ident == "Option" {
                    opt_args.push(ConstructorOptArg {
                        ident: named_field.ident.unwrap(),
                    });
                } else if named_field.ident.as_ref().unwrap() == "timestamp" {
                    has_timestamp = true;
                    continue;
                } else {
                    let arg = ConstructorArg {
                        ident: named_field.ident.unwrap(),
                        r#type: named_field.ty,
                    };
                    cons_args.push(arg);
                }
            }
            _ => {
                if named_field.ident.as_ref().unwrap() == "timestamp" {
                    has_timestamp = true;
                    continue;
                } else {
                    cons_args.push(ConstructorArg {
                        ident: named_field.ident.unwrap(),
                        r#type: named_field.ty,
                    });
                }
            }
        }
    }
    (cons_args, opt_args, has_timestamp)
}

#[proc_macro_derive(APIRequestInit)]
pub fn api_request_init_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    match data {
        syn::Data::Struct(data_struct) => parse_struct(ident, data_struct),
        syn::Data::Enum(_data_enum) => {
            unimplemented!()
        }
        syn::Data::Union(_) => {
            unimplemented!()
        }
    }
}

fn parse_struct(ident: Ident, data: DataStruct) -> TokenStream {
    match data.fields {
        syn::Fields::Named(named_fields) => {
            let (cons_args, opt_args, has_timestamp) =
                api_request_init_parse_struct_named_field(named_fields);

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

            quote! {
               impl #ident {
                   pub fn init(#(#cons_args),*) -> Self {
                       #ident {
                           #(#required_field_names),*
                           #comma_after_required
                           #(#opt_args),*
                           #comma_after_opts
                           #init_timestamp
                       }
                   }
               }
            }
            .into()
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
    }
}
