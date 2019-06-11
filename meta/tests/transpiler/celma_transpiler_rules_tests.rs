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

#[cfg(test)]
mod tests_and {
    use quote::quote;

    use celma_core::parser::parser::Parse;
    use celma_core::parser::response::Response::Success;
    use celma_core::stream::char_stream::CharStream;
    use celma_lang::meta::parser::celma_parsec_rules;
    use celma_lang::meta::transpiler::Transpile;

    #[test]
    fn it_parse_two_char_rules() {
        let response = celma_parsec_rules()
            .parse(CharStream::new(
                "let a:{Vec<char>} = b let b:{Vec<char>} = 'b'+",
            ))
            .fmap(|ast| ast.transpile());

        match response {
            Success(ast, _, _) => assert_eq!(
                ast.to_string(),
                quote!(
                    use celma_core::parser::and::AndOperation;
                    use celma_core::parser::fmap::FMapOperation;
                    use celma_core::parser::or::OrOperation;
                    use celma_core::parser::parser::Parse;
                    use celma_core::parser::repeat::RepeatOperation;

                    fn a<'a, S: 'a>() -> impl celma_core::parser::parser::Parse<Vec<char>, S>
                                             + celma_core::parser::parser::Combine<Vec<char> >
                                             + Clone
                                             + 'a
                    where
                        S: celma_core::stream::stream::Stream<Item = char>,
                    {
                        celma_core::parser::lazy::lazy(|| b())
                    }

                    fn b<'a, S: 'a>() -> impl celma_core::parser::parser::Parse<Vec<char>, S>
                                             + celma_core::parser::parser::Combine<Vec<char> >
                                             + Clone
                                             + 'a
                    where
                        S: celma_core::stream::stream::Stream<Item = char>,
                    {
                        celma_core::parser::char::char('b').rep()
                    }
                )
                .to_string()
            ),
            _ => assert_eq!(true, false),
        };
    }

}
