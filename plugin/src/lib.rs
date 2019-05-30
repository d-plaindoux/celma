extern crate proc_macro;

mod parser;

use proc_macro::TokenStream;

use quote::quote;

#[proc_macro]
pub fn parsec<'a>(input: TokenStream) -> TokenStream {
    println!("Parse [{}]", input.to_string());

    quote!(celma::parser::core::eos()).into()
}