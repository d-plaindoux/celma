/*
   Copyright 2019 Didier Plaindoux

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;

use celma_core::parser::parser::Parse;
use celma_core::parser::response::Response::{Reject, Success};
use celma_core::stream::char_stream::CharStream;
use celma_core::stream::stream::Stream;
use celma_lang::meta::parser::{celma_parsec, celma_parsec_rules};

#[proc_macro]
pub fn parsec(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let result = celma_parsec()
        .parse(CharStream::new(source.as_str()))
        .fmap(|ast| ast.transpile());

    match result {
        Success(_, _, _) => (),
        Reject(_, _) => (),
    }

    quote!(celma_core::parser::core::eos()).into()
}

#[proc_macro]
pub fn parsec_rules(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let result = celma_parsec_rules().parse(CharStream::new(source.as_str()));

    match result {
        Success(_, _, _) => (),
        Reject(s, _) => panic!(format!("Error at {:?}", s.position())),
    }

    quote!(celma_core::parser::core::eos()).into()
}
