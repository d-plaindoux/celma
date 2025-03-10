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

#[cfg(test)]
mod tests_and {
    use proc_macro2::TokenStream;
    use quote::quote;

    use celma_v0_core::parser::response::Response::Success;
    use celma_v0_core::parser::specs::Parse;
    use celma_v0_core::stream::char_stream::CharStream;
    use celma_v0_parser::parser::celma_parsec_rules;
    use celma_v0_parser::transpiler::Transpile;

    #[test]
    fn it_parse_two_char_rules() {
        let response = celma_parsec_rules()
            .parse(CharStream::new(
                "let a:{Vec<char>} = b let b:{Vec<char>} = 'b'+",
            ))
            .map(|ast| ast.transpile());

        match response {
            Success(ast, _, _) => assert_eq!(ast.unwrap().to_string(), expect_code().to_string()),
            _ => assert_eq!(true, false),
        };
    }

    #[rustfmt::skip]
    fn expect_code() -> TokenStream {
        quote!(
            pub fn a<'a, S: 'a>() -> impl celma_v0_core::parser::specs::Parse<Vec<char>, S>
                   + celma_v0_core::parser::specs::Combine<Vec<char> >
                   + 'a
            where
                S: celma_v0_core::stream::specs::Stream<Item = char>,
            {
                use celma_v0_core::parser::a_try::a_try;
                use celma_v0_core::parser::and::AndOperation;
                use celma_v0_core::parser::check::check;
                use celma_v0_core::parser::lookahead::lookahead;
                use celma_v0_core::parser::map::MapOperation;
                use celma_v0_core::parser::not::NotOperation;
                use celma_v0_core::parser::option::OptionalOperation;
                use celma_v0_core::parser::or::OrOperation;
                use celma_v0_core::parser::repeat::RepeatOperation;
                use celma_v0_core::parser::specs::Parse;

                celma_v0_core::parser::core::parser(celma_v0_core::parser::lazy::lazy(|| b()))
            }

            pub fn b<'a, S: 'a>() -> impl celma_v0_core::parser::specs::Parse<Vec<char>, S>
                   + celma_v0_core::parser::specs::Combine<Vec<char> >
                   + 'a
            where
                S: celma_v0_core::stream::specs::Stream<Item = char>,
            {
                use celma_v0_core::parser::a_try::a_try;
                use celma_v0_core::parser::and::AndOperation;
                use celma_v0_core::parser::check::check;
                use celma_v0_core::parser::lookahead::lookahead;
                use celma_v0_core::parser::map::MapOperation;
                use celma_v0_core::parser::not::NotOperation;
                use celma_v0_core::parser::option::OptionalOperation;
                use celma_v0_core::parser::or::OrOperation;
                use celma_v0_core::parser::repeat::RepeatOperation;
                use celma_v0_core::parser::specs::Parse;

                celma_v0_core::parser::core::parser(celma_v0_core::parser::char::a_char('b').rep())
            }
        )
    }
}
