extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;

use celma::parser::parser::Parse;
use celma::stream::char_stream::CharStream;

mod parser;

#[proc_macro]
pub fn parsec(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let result = crate::parser::parsec().parse(CharStream::new(source.as_str()));
    println!("Parse [{}]", result.fold(|_, _, _| true, |_| false));

    quote!(celma::parser::core::eos()).into()
}
