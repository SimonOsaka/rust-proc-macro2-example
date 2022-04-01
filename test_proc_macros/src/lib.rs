use proc_macro::TokenStream;
use proc_macro2::{Punct, Spacing};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    token::Paren,
    DeriveInput, Fields, FieldsUnnamed, ItemFn, ItemStruct, Lit, LitStr, Signature, Token,
};

extern crate proc_macro;
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
