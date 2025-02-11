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
    use celma_core::parser::response::Response::Success;
    use celma_core::parser::specs::Parse;
    use celma_core::stream::char_stream::CharStream;
    use celma_lang_v0_ast::syntax::ASTParsec::{
        PBind, PChoice, PCode, PMap, POptional, PRepeat, PSequence,
    };
    use celma_lang_v0_parser::parser::celma_parsec;

    #[test]
    fn it_parse_one_character() {
        let response = celma_parsec().parse(CharStream::new("{char('a')}"));

        match response {
            Success(ast, _, _) => assert_eq!(ast, PCode(String::from("char(\'a\')"))),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_two_characters() {
        let response = celma_parsec().parse(CharStream::new("{char('a')} {char('b')}"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PSequence(
                    Box::new(PCode(String::from("char(\'a\')"))),
                    Box::new(PCode(String::from("char(\'b\')"))),
                )
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_one_character_with_choice() {
        let response = celma_parsec().parse(CharStream::new("{char('a')} | {char('b')}"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PChoice(
                    Box::new(PCode(String::from("char(\'a\')"))),
                    Box::new(PCode(String::from("char(\'b\')"))),
                )
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_one_character_with_binding() {
        let response = celma_parsec().parse(CharStream::new("c={char('a')}"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PBind(
                    String::from("c"),
                    Box::new(PCode(String::from("char(\'a\')"))),
                )
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_one_optional_character_with_binding() {
        let response = celma_parsec().parse(CharStream::new("c={char('a')}?"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PBind(
                    String::from("c"),
                    Box::new(POptional(Box::new(PCode(String::from("char(\'a\')"))))),
                )
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_one_optional_repeatable_character_with_binding() {
        let response = celma_parsec().parse(CharStream::new("c={char('a')}*"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PBind(
                    String::from("c"),
                    Box::new(PRepeat(true, Box::new(PCode(String::from("char(\'a\')"))))),
                )
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_one_repeatable_character_with_binding() {
        let response = celma_parsec().parse(CharStream::new("c={char('a')}+"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PBind(
                    String::from("c"),
                    Box::new(PRepeat(false, Box::new(PCode(String::from("char(\'a\')"))))),
                )
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_two_repeatable_character_with_binding() {
        let response = celma_parsec().parse(CharStream::new("a={char('a')}+ b={char('b')}+"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PSequence(
                    Box::new(PBind(
                        String::from("a"),
                        Box::new(PRepeat(false, Box::new(PCode(String::from("char(\'a\')"))))),
                    )),
                    Box::new(PBind(
                        String::from("b"),
                        Box::new(PRepeat(false, Box::new(PCode(String::from("char(\'b\')"))))),
                    )),
                )
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_a_repeatable_character_with_binding_with_choice() {
        let response = celma_parsec().parse(CharStream::new("a={char('a')}+ | b={char('b')}+"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PChoice(
                    Box::new(PBind(
                        String::from("a"),
                        Box::new(PRepeat(false, Box::new(PCode(String::from("char(\'a\')"))))),
                    )),
                    Box::new(PBind(
                        String::from("b"),
                        Box::new(PRepeat(false, Box::new(PCode(String::from("char(\'b\')"))))),
                    )),
                )
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_a_character_with_map() {
        let response = celma_parsec().parse(CharStream::new("a={char('a')} -> { Result(a) }"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PMap(
                    Box::new(PBind(
                        String::from("a"),
                        Box::new(PCode(String::from("char(\'a\')"))),
                    )),
                    String::from(" Result(a) "),
                )
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_a_mapped_character_with_map() {
        let response = celma_parsec().parse(CharStream::new(
            "a=({char('a')} -> { 'a' }) -> { Result(a) }",
        ));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PMap(
                    Box::new(PBind(
                        String::from("a"),
                        Box::new(PMap(
                            Box::new(PCode(String::from("char(\'a\')"))),
                            String::from(" 'a' ")
                        )),
                    )),
                    String::from(" Result(a) "),
                )
            ),
            _ => assert_eq!(true, false),
        };
    }
}
