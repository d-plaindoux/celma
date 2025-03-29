/*
   Copyright 2019-2025 Didier Plaindoux

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

use celma_v0_core::parser::response::Response;
use celma_v0_core::parser::response::Response::{Reject, Success};
use celma_v0_core::parser::specs::Parse;
use celma_v0_core::stream::char_stream::CharStream;
use celma_v0_core::stream::position::CharPosition;
use celma_v0_core::stream::specs::Stream;
use celma_v0_parser::parser::{celma_parsec, celma_parsec_rules};
use celma_v0_parser::transpiler::Transpile;
use syn::Error;

#[proc_macro]
pub fn parsec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let result = celma_parsec()
        .parse(CharStream::new(source.as_str()))
        .map(|ast| ast.transpile());

    conclude_parsing(result)
}

#[proc_macro]
pub fn parsec_rules(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let result = celma_parsec_rules()
        .parse(CharStream::new(source.as_str()))
        .map(|ast| ast.transpile());

    conclude_parsing(result)
}

fn conclude_parsing(
    result: Response<Result<proc_macro2::TokenStream, Error>, CharStream<CharPosition>>,
) -> proc_macro::TokenStream {
    match result {
        Success(code, _, _) => match code {
            Ok(code) => code.into(),
            Err(err) => panic!("{}", err.into_compile_error()),
        },
        Reject(s, _) => panic!("Parse error at {:?}", s.position()),
    }
}
