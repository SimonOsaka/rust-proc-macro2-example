fn main() {
    // let pre = vec![
    let t1 = proc_macro2::TokenTree::Ident(proc_macro2::Ident::new(
        "fn",
        proc_macro2::Span::call_site(),
    ));
    let t2 = proc_macro2::TokenTree::Ident(proc_macro2::Ident::new(
        "foo",
        proc_macro2::Span::call_site(),
    ));
    let t3 = proc_macro2::TokenTree::Group(proc_macro2::Group::new(
        proc_macro2::Delimiter::Parenthesis,
        proc_macro2::TokenStream::new(),
    ));
    let t4 = proc_macro2::TokenTree::Group(proc_macro2::Group::new(
        proc_macro2::Delimiter::Brace,
        proc_macro2::TokenStream::new(),
    ));
    // ];
    println!("{}{}{}{}", t1, t2, t3, t4);
}
