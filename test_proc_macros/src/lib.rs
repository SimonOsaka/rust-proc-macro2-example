use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    token::Paren,
    DeriveInput, Fields, FieldsUnnamed, ItemFn, ItemStruct, LitStr, Signature, Token,
};

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let output = quote! {
        impl #name {
            fn ok(&self){
                println!("Hi {}, i'm ok!", self.name);
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn my_attribute_struct(
    _args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let class = parse_macro_input!(item as ItemStruct);
    let name = &class.ident;
    let output = quote! {
        #class

        impl #name {
            fn drive(&self) {
                println!("Car no {}", self.no);
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn my_attribute_fn(
    _args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ItemFn {
        attrs: _,
        vis,
        sig,
        block,
    } = parse_macro_input!(item as ItemFn);

    let Signature {
        output: return_type,
        inputs: params,
        unsafety,
        asyncness,
        constness,
        abi,
        ident,
        generics:
            syn::Generics {
                params: gen_params,
                where_clause,
                ..
            },
        ..
    } = sig;

    let output = quote! {

        #vis #constness #unsafety #asyncness #abi fn #ident<#gen_params>(#params) #return_type
        #where_clause
        {
            println!("begin");
            let r = || #block;
            let r = r();
            println!("after");
            r
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn add_field(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: ItemStruct = syn::parse_macro_input!(input);
    let fields = if let Fields::Named(fields) = &mut item.fields {
        fields
    } else {
        unreachable!()
    };
    // fields.named.push(Field {
    //     attrs: vec![],
    //     vis: Visibility::Inherited,
    //     ident: Some(format_ident!("d")),
    //     colon_token: Some(<Token![:]>::default()),
    //     ty: parse_quote!(String),
    // });
    fields.named.push(
        syn::Field::parse_named
            .parse2(quote! { pub d: String })
            .unwrap(),
    );

    item.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn add_tuple(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: ItemStruct = syn::parse_macro_input!(input);

    let field = syn::Field::parse_unnamed.parse2(quote! { String }).unwrap();

    if let Fields::Unnamed(fields) = &mut item.fields {
        fields.unnamed.push(field);
    } else {
        let mut unnamed = Punctuated::new();
        unnamed.push(field);
        let expanded_fields = Fields::Unnamed(FieldsUnnamed {
            paren_token: Paren::default(),
            unnamed,
        });
        item.fields = expanded_fields;
    };

    item.into_token_stream().into()
}

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as MyItem);

    // let output = quote! {};
    // TokenStream::from(output)
    quote!(#ast).into_token_stream().into()
}
struct MyPair {
    k: LitStr,
    v: LitStr,
}
impl syn::parse::Parse for MyPair {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let k: syn::Ident = input.parse()?;
        let k = LitStr::new(&k.to_string(), k.span());
        let _colon: Token![=] = input.parse()?;
        let v: LitStr = input.parse()?;
        Ok(MyPair { k, v })
    }
}
impl ToTokens for MyPair {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let MyPair { k, v } = self;
        tokens.extend(quote! {
            map.insert(#k, #v);
        });
    }
}
struct MyItem {
    pairs: Vec<MyPair>,
}
impl Parse for MyItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut pairs: Vec<MyPair> = Vec::new();
        loop {
            let pair: MyPair = input.parse()?;
            pairs.push(pair);
            if !input.is_empty() {
                let _comma: Token![,] = input.parse()?;
            } else {
                break;
            }
        }

        Ok(MyItem { pairs })
    }
}
impl ToTokens for MyItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(quote! {
            let mut map = std::collections::HashMap::new();
        });
        for item in &self.pairs {
            tokens.extend(quote! {
                #item
            });
        }
    }
}

/// ErrorMessage
#[proc_macro_derive(ErrorMessage, attributes(error_message))]
pub fn error_message_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    //
    let name = &ast.ident;
    //
    let enum_data = if let syn::Data::Enum(data) = &ast.data {
        data
    } else {
        panic!("{} is not an enum", name);
    };
    //
    let mut stream = proc_macro2::TokenStream::new();

    for variant_data in &enum_data.variants {
        let variant_name = &variant_data.ident;
        let function_name_ref = Ident::new(
            &format!("as_{}", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_ref = format!(
            "Optionally returns references to the inner fields if this is a `{}::{}`, otherwise `None`",
            name,
            variant_name,
        );
        let function_name_val = Ident::new(
            &format!("into_{}", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_val = format!(
            "Returns the inner fields if this is a `{}::{}`, otherwise returns back the enum in the `Err` case of the result",
            name,
            variant_name,
        );
        //
        let tokens = match &variant_data.fields {
            syn::Fields::Unit => {
                println!("Unit {}", variant_name.to_string());
                unit_fields_return(name, variant_name, &function_name_ref, &doc_ref)
            }
            syn::Fields::Unnamed(unnamed) => {
                println!("Unnamed {}", name.to_string());
                unnamed_fields_return(
                    name,
                    variant_name,
                    (&function_name_ref, &doc_ref),
                    (&function_name_val, &doc_val),
                    &unnamed,
                )
            }
            syn::Fields::Named(named) => {
                println!("Named {}", name.to_string());
                named_fields_return(
                    name,
                    variant_name,
                    (&function_name_ref, &doc_ref),
                    (&function_name_val, &doc_val),
                    &named,
                )
            }
        };
        stream.extend(tokens);
    }
    quote! {
        impl #name {
            #stream
        }
    }
    .into()
}

fn unit_fields_return(
    name: &syn::Ident,
    variant_name: &syn::Ident,
    function_name: &Ident,
    doc: &str,
) -> proc_macro2::TokenStream {
    quote!(
        #[doc = #doc ]
        pub fn #function_name(&self) -> Option<()> {
            match self {
                #name::#variant_name => {
                    Some(())
                }
                _ => None
            }
        }
    )
}

/// returns first the types to return, the match names, and then tokens to the field accesses
fn unnamed_fields_return(
    name: &syn::Ident,
    variant_name: &syn::Ident,
    (function_name_ref, doc_ref): (&Ident, &str),
    (function_name_val, doc_val): (&Ident, &str),
    fields: &syn::FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let (returns_ref, returns_val, matches, accesses_ref, accesses_val) = match fields.unnamed.len()
    {
        1 => {
            let field = fields.unnamed.first().expect("no fields on type");

            let returns = &field.ty;
            let returns_ref = quote!(&#returns);
            let returns_val = quote!(#returns);
            let matches = quote!(inner);
            let accesses_ref = quote!(&inner);
            let accesses_val = quote!(inner);

            (
                returns_ref,
                returns_val,
                matches,
                accesses_ref,
                accesses_val,
            )
        }
        0 => (quote!(()), quote!(()), quote!(), quote!(()), quote!(())),
        _ => {
            let mut returns_ref = proc_macro2::TokenStream::new();
            let mut returns_val = proc_macro2::TokenStream::new();
            let mut matches = proc_macro2::TokenStream::new();
            let mut accesses_ref = proc_macro2::TokenStream::new();
            let mut accesses_val = proc_macro2::TokenStream::new();

            for (i, field) in fields.unnamed.iter().enumerate() {
                let rt = &field.ty;
                let match_name = Ident::new(&format!("match_{}", i), Span::call_site());
                returns_ref.extend(quote!(&#rt,));
                returns_val.extend(quote!(#rt,));
                matches.extend(quote!(#match_name,));
                accesses_ref.extend(quote!(&#match_name,));
                accesses_val.extend(quote!(#match_name,));
            }

            (
                quote!((#returns_ref)),
                quote!((#returns_val)),
                quote!(#matches),
                quote!((#accesses_ref)),
                quote!((#accesses_val)),
            )
        }
    };

    quote!(
        #[doc = #doc_ref ]
        pub fn #function_name_ref(&self) -> Option<#returns_ref> {
            match self {
                #name::#variant_name(#matches) => {
                    Some(#accesses_ref)
                }
                _ => None
            }
        }

        #[doc = #doc_val ]
        pub fn #function_name_val(self) -> ::core::result::Result<#returns_val, Self> {
            match self {
                #name::#variant_name(#matches) => {
                    Ok(#accesses_val)
                },
                _ => Err(self)
            }
        }
    )
}

/// returns first the types to return, the match names, and then tokens to the field accesses
fn named_fields_return(
    name: &syn::Ident,
    variant_name: &syn::Ident,
    (function_name_ref, doc_ref): (&Ident, &str),
    (function_name_val, doc_val): (&Ident, &str),
    fields: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let (returns_ref, returns_val, matches, accesses_ref, accesses_val) = match fields.named.len() {
        1 => {
            let field = fields.named.first().expect("no fields on type");
            let match_name = field.ident.as_ref().expect("expected a named field");

            let returns = &field.ty;
            let returns_ref = quote!(&#returns);
            let returns_val = quote!(#returns);
            let matches = quote!(#match_name);
            let accesses_ref = quote!(&#match_name);
            let accesses_val = quote!(#match_name);

            (
                returns_ref,
                returns_val,
                matches,
                accesses_ref,
                accesses_val,
            )
        }
        0 => (quote!(()), quote!(()), quote!(), quote!(()), quote!(())),
        _ => {
            let mut returns_ref = proc_macro2::TokenStream::new();
            let mut returns_val = proc_macro2::TokenStream::new();
            let mut matches = proc_macro2::TokenStream::new();
            let mut accesses_ref = proc_macro2::TokenStream::new();
            let mut accesses_val = proc_macro2::TokenStream::new();

            for field in fields.named.iter() {
                let rt = &field.ty;
                let match_name = field.ident.as_ref().expect("expected a named field");

                returns_ref.extend(quote!(&#rt,));
                returns_val.extend(quote!(#rt,));
                matches.extend(quote!(#match_name,));
                accesses_ref.extend(quote!(&#match_name,));
                accesses_val.extend(quote!(#match_name,));
            }

            (
                quote!((#returns_ref)),
                quote!((#returns_val)),
                quote!(#matches),
                quote!((#accesses_ref)),
                quote!((#accesses_val)),
            )
        }
    };

    quote!(
        #[doc = #doc_ref ]
        pub fn #function_name_ref(&self) -> Option<#returns_ref> {
            match self {
                #name::#variant_name{ #matches } => {
                    Some(#accesses_ref)
                }
                _ => None
            }
        }

        #[doc = #doc_val ]
        pub fn #function_name_val(self) -> ::core::result::Result<#returns_val, Self> {
            match self {
                #name::#variant_name{ #matches } => {
                    Ok(#accesses_val)
                }
                _ => Err(self)
            }
        }
    )
}
