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
    use celma_v0_ast::syntax::ASTParsec::{
        PBind, PChoice, PCode, PEpsilon, PMap, POptional, PRepeat, PSequence,
    };
    use celma_v0_core::parser::response::Response::Success;
    use celma_v0_core::parser::specs::Parse;
    use celma_v0_core::stream::char_stream::CharStream;
    use celma_v0_parser::parser::celma_parsec;

    #[test]
    fn it_parse_one_character() {
        let response = celma_parsec().parse(CharStream::new("{char('a')}"));

        match response {
            Success(ast, _, _) => assert_eq!(ast, PCode(String::from("char(\'a\')"))),
            _ => panic!(),
        };
    }

    #[test]
    fn it_parse_two_characters() {
        let response = celma_parsec().parse(CharStream::new("{char('a')} {char('b')}"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PSequence(
                    PCode(String::from("char(\'a\')")).wrap(),
                    PCode(String::from("char(\'b\')")).wrap(),
                )
            ),
            _ => panic!(),
        };
    }

    #[test]
    fn it_parse_one_character_with_choice() {
        let response = celma_parsec().parse(CharStream::new("{char('a')} | {char('b')}"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PChoice(
                    PCode(String::from("char(\'a\')")).wrap(),
                    PCode(String::from("char(\'b\')")).wrap(),
                )
            ),
            _ => panic!(),
        };
    }

    #[test]
    fn it_parse_one_character_with_binding() {
        let response = celma_parsec().parse(CharStream::new("c={char('a')}"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PBind(String::from("c"), PCode(String::from("char(\'a\')")).wrap(),)
            ),
            _ => panic!(),
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
                    POptional(PCode(String::from("char(\'a\')")).wrap()).wrap(),
                )
            ),
            _ => panic!(),
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
                    PRepeat(true, PCode(String::from("char(\'a\')")).wrap()).wrap(),
                )
            ),
            _ => panic!(),
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
                    PRepeat(false, PCode(String::from("char(\'a\')")).wrap()).wrap(),
                )
            ),
            _ => panic!(),
        };
    }

    #[test]
    fn it_parse_two_repeatable_character_with_binding() {
        let response = celma_parsec().parse(CharStream::new("a={char('a')}+ b={char('b')}+"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PSequence(
                    PBind(
                        String::from("a"),
                        PRepeat(false, PCode(String::from("char(\'a\')")).wrap()).wrap(),
                    )
                    .wrap(),
                    PBind(
                        String::from("b"),
                        PRepeat(false, PCode(String::from("char(\'b\')")).wrap()).wrap(),
                    )
                    .wrap(),
                )
            ),
            _ => panic!(),
        };
    }

    #[test]
    fn it_parse_a_repeatable_character_with_binding_with_choice() {
        let response = celma_parsec().parse(CharStream::new("a={char('a')}+ | b={char('b')}+"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PChoice(
                    PBind(
                        String::from("a"),
                        PRepeat(false, PCode(String::from("char(\'a\')")).wrap()).wrap(),
                    )
                    .wrap(),
                    PBind(
                        String::from("b"),
                        PRepeat(false, PCode(String::from("char(\'b\')")).wrap()).wrap(),
                    )
                    .wrap(),
                )
            ),
            _ => panic!(),
        };
    }

    #[test]
    fn it_parse_a_character_with_map() {
        let response = celma_parsec().parse(CharStream::new("a={char('a')} -> { Result(a) }"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                PMap(
                    PBind(String::from("a"), PCode(String::from("char(\'a\')")).wrap(),).wrap(),
                    String::from(" Result(a) "),
                )
            ),
            _ => panic!(),
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
                    PBind(
                        String::from("a"),
                        PMap(
                            PCode(String::from("char(\'a\')")).wrap(),
                            String::from(" 'a' ")
                        )
                        .wrap(),
                    )
                    .wrap(),
                    String::from(" Result(a) "),
                )
            ),
            _ => panic!(),
        };
    }

    #[test]
    fn it_parse_epsilon() {
        let response = celma_parsec().parse(CharStream::new("()"));

        match response {
            Success(ast, _, _) => assert_eq!(ast, PEpsilon(),),
            _ => panic!(),
        };
    }

    #[test]
    fn it_parse_binded_epsilon() {
        let response = celma_parsec().parse(CharStream::new("a=()"));

        match response {
            Success(ast, _, _) => assert_eq!(ast, PBind(String::from("a"), PEpsilon().wrap()),),
            _ => panic!(),
        };
    }
}
