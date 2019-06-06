extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;

use celma_core::parser::parser::Parse;
use celma_core::parser::response::Response::{Reject, Success};
use celma_core::stream::char_stream::CharStream;
use celma_lang::meta::parser::celma_language;

#[proc_macro]
pub fn parsec(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let result = celma_language().parse(CharStream::new(source.as_str()));

    match result {
        Success(_, _, _) => (),
        Reject(_, _) => (),
    }

    quote!(celma_core::parser::core::eos()).into()
}
