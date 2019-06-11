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
    use celma_core::parser::parser::Parse;
    use celma_core::parser::response::Response::Success;
    use celma_core::stream::char_stream::CharStream;
    use celma_lang::meta::parser::celma_parsec;

    #[test]
    fn it_transpile_one_character() {
        let response = celma_parsec()
            .parse(CharStream::new("'a'"))
            .fmap(|ast| ast.transpile());

        match response {
            Success((_, ast), _, _) => assert_eq!(ast, String::from("char('a')")),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_two_characters() {
        let response = celma_parsec()
            .parse(CharStream::new("'a' 'b'"))
            .fmap(|ast| ast.transpile());

        match response {
            Success((_, ast), _, _) => {
                assert_eq!(ast, String::from("char('a').and_right(char('b'))"))
            }
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_two_characters_bind_left() {
        let response = celma_parsec()
            .parse(CharStream::new("a='a' 'b'"))
            .fmap(|ast| ast.transpile());

        match response {
            Success((params, ast), _, _) => {
                assert_eq!(ast, String::from("char('a').and_left(char('b'))"));
                assert_eq!(params, String::from("a"));
            }
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_two_bound_characters() {
        let response = celma_parsec()
            .parse(CharStream::new("a='a' b='b'"))
            .fmap(|ast| ast.transpile());

        match response {
            Success((params, _), _, _) => assert_eq!(params, String::from("(a,b)")),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_three_characters_with_two_binds() {
        let response = celma_parsec()
            .parse(CharStream::new("a='a' 'b' c='c'"))
            .fmap(|ast| ast.transpile());

        match response {
            Success((params, ast), _, _) => {
                assert_eq!(
                    ast,
                    String::from("char('a').and(char('b').and_right(char('c')))")
                );
                assert_eq!(params, String::from("(a,c)"))
            }
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_three_bound_characters() {
        let response = celma_parsec()
            .parse(CharStream::new("a='a' b='b' c='c'"))
            .fmap(|ast| ast.transpile());

        match response {
            Success((params, ast), _, _) => {
                assert_eq!(ast, String::from("char('a').and(char('b').and(char('c')))"));
                assert_eq!(params, String::from("(a,(b,c))"))
            }
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_transpile_one_choice_characters() {
        let response = celma_parsec()
            .parse(CharStream::new("'a' | 'b'"))
            .fmap(|ast| ast.transpile());

        match response {
            Success((_, ast), _, _) => assert_eq!(ast, String::from("char('a').or(char('b'))")),
            _ => assert_eq!(true, false),
        };
    }
}
