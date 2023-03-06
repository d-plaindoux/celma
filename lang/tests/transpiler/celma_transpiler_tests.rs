/*
   Copyright 2019-2023 Didier Plaindoux

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
    use celma_core::parser::parser::Parse;
    use celma_core::parser::response::Response::Success;
    use celma_core::stream::char_stream::CharStream;
    use celma_lang::meta::parser::celma_parsec;
    use celma_lang::meta::transpiler::TranspileBody;
    use quote::quote;

    #[test]
    fn it_transpile_one_character() {
        let response = celma_parsec()
            .parse(CharStream::new("'a'"))
            .fmap(|ast| ast.transpile_body());

        match response {
            Success((_, ast), _, _) => assert_eq!(
                ast.to_string(),
                quote!(celma_core::parser::char::char('a')).to_string()
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_two_characters() {
        let response = celma_parsec()
            .parse(CharStream::new("'a' 'b'"))
            .fmap(|ast| ast.transpile_body());

        match response {
            Success((_, ast), _, _) => assert_eq!(
                ast.to_string(),
                quote!(celma_core::parser::char::char('a')
                    .and_right(celma_core::parser::char::char('b')))
                .to_string()
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_two_characters_with_try_on_the_second_one() {
        let response = celma_parsec()
            .parse(CharStream::new("'a' !'b'"))
            .fmap(|ast| ast.transpile_body());

        match response {
            Success((_, ast), _, _) => assert_eq!(
                ast.to_string(),
                quote!(celma_core::parser::char::char('a')
                    .and_right(a_try(celma_core::parser::char::char('b'))))
                    .to_string()
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_two_characters_with_lookahead_on_the_second_one() {
        let response = celma_parsec()
            .parse(CharStream::new("'a' /'b'"))
            .fmap(|ast| ast.transpile_body());

        match response {
            Success((_, ast), _, _) => assert_eq!(
                ast.to_string(),
                quote!(celma_core::parser::char::char('a')
                    .and_right(lookahead(celma_core::parser::char::char('b'))))
                    .to_string()
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_two_characters_bind_left() {
        let response = celma_parsec()
            .parse(CharStream::new("a='a' 'b'"))
            .fmap(|ast| ast.transpile_body());

        match response {
            Success((params, ast), _, _) => {
                assert_eq!(
                    ast.to_string(),
                    quote!(celma_core::parser::char::char('a')
                        .and_left(celma_core::parser::char::char('b')))
                    .to_string()
                );
                assert_eq!(params, Some(String::from("a")));
            }
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_two_bound_characters() {
        let response = celma_parsec()
            .parse(CharStream::new("a='a' b='b'"))
            .fmap(|ast| ast.transpile_body());

        match response {
            Success((params, _), _, _) => assert_eq!(params, Some(String::from("(a,b)"))),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_three_characters_with_two_binds() {
        let response = celma_parsec()
            .parse(CharStream::new("a='a' 'b' c='c'"))
            .fmap(|ast| ast.transpile_body());

        match response {
            Success((params, ast), _, _) => {
                assert_eq!(
                    ast.to_string(),
                    quote!(celma_core::parser::char::char('a').and(
                        celma_core::parser::char::char('b')
                            .and_right(celma_core::parser::char::char('c'))
                    ))
                    .to_string()
                );
                assert_eq!(params, Some(String::from("(a,c)")))
            }
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_three_bound_characters() {
        let response = celma_parsec()
            .parse(CharStream::new("a='a' b='b' c='c'"))
            .fmap(|ast| ast.transpile_body());

        match response {
            Success((params, ast), _, _) => {
                assert_eq!(
                    ast.to_string(),
                    quote!(celma_core::parser::char::char('a').and(
                        celma_core::parser::char::char('b')
                            .and(celma_core::parser::char::char('c'))
                    ))
                    .to_string()
                );
                assert_eq!(params, Some(String::from("(a,(b,c))")))
            }
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_one_choice_characters() {
        let response = celma_parsec()
            .parse(CharStream::new("'a' | 'b'"))
            .fmap(|ast| ast.transpile_body());

        match response {
            Success((_, ast), _, _) => assert_eq!(
                ast.to_string(),
                quote!(celma_core::parser::char::char('a').or(celma_core::parser::char::char('b')))
                    .to_string()
            ),
            _ => assert_eq!(true, false),
        };
    }
}
