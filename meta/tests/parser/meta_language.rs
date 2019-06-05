#[cfg(test)]
mod tests_and {
    use celma_core::parser::parser::Parse;
    use celma_core::parser::response::Response::{Reject, Success};
    use celma_core::stream::char_stream::CharStream;
    use celma_lang::parser::ASTParsec::{PBind, PChoice, PCode, POptional, PRepeat, PSequence, PMap};
    use celma_lang::parser::celma_language;

    #[test]
    fn it_parse_one_character() {
        let response = celma_language().parse(CharStream::new("{char('a')}"));

        match response {
            Success(ast, _, _) =>
                assert_eq!(
                    ast,
                    PCode(String::from("char(\'a\')"))
                ),
            Reject(_) =>
                assert_eq!(true, false)
        };
    }

    #[test]
    fn it_parse_two_characters() {
        let response = celma_language().parse(CharStream::new("{char('a')} {char('b')}"));

        match response {
            Success(ast, _, _) =>
                assert_eq!(
                    ast,
                    PSequence(
                        Box::new(PCode(String::from("char(\'a\')"))),
                        Box::new(PCode(String::from("char(\'b\')"))))
                ),
            Reject(_) =>
                assert_eq!(true, false)
        };
    }

    #[test]
    fn it_parse_one_character_with_choice() {
        let response = celma_language().parse(CharStream::new("{char('a')} | {char('b')}"));

        match response {
            Success(ast, _, _) =>
                assert_eq!(
                    ast,
                    PChoice(
                        Box::new(PCode(String::from("char(\'a\')"))),
                        Box::new(PCode(String::from("char(\'b\')"))))
                ),
            Reject(_) =>
                assert_eq!(true, false)
        };
    }

    #[test]
    fn it_parse_one_character_with_binding() {
        let response = celma_language().parse(CharStream::new("c={char('a')}"));

        match response {
            Success(ast, _, _) =>
                assert_eq!(
                    ast,
                    PBind(String::from("c"), Box::new(PCode(String::from("char(\'a\')"))))
                ),
            Reject(_) =>
                assert_eq!(true, false)
        };
    }

    #[test]
    fn it_parse_one_optional_character_with_binding() {
        let response = celma_language().parse(CharStream::new("c={char('a')}?"));

        match response {
            Success(ast, _, _) =>
                assert_eq!(
                    ast,
                    PBind(String::from("c"), Box::new(POptional(Box::new(PCode(String::from("char(\'a\')"))))))
                ),
            Reject(_) =>
                assert_eq!(true, false)
        };
    }

    #[test]
    fn it_parse_one_optional_repeatable_character_with_binding() {
        let response = celma_language().parse(CharStream::new("c={char('a')}*"));

        match response {
            Success(ast, _, _) =>
                assert_eq!(
                    ast,
                    PBind(String::from("c"), Box::new(PRepeat(true, Box::new(PCode(String::from("char(\'a\')"))))))
                ),
            Reject(_) =>
                assert_eq!(true, false)
        };
    }

    #[test]
    fn it_parse_one_repeatable_character_with_binding() {
        let response = celma_language().parse(CharStream::new("c={char('a')}+"));

        match response {
            Success(ast, _, _) =>
                assert_eq!(
                    ast,
                    PBind(String::from("c"), Box::new(PRepeat(false, Box::new(PCode(String::from("char(\'a\')"))))))
                ),
            Reject(_) =>
                assert_eq!(true, false)
        };
    }

    #[test]
    fn it_parse_two_repeatable_character_with_binding() {
        let response = celma_language().parse(CharStream::new("a={char('a')}+ b={char('b')}+"));

        match response {
            Success(ast, _, _) =>
                assert_eq!(
                    ast,
                    PSequence(
                        Box::new(PBind(String::from("a"), Box::new(PRepeat(false, Box::new(PCode(String::from("char(\'a\')"))))))),
                        Box::new(PBind(String::from("b"), Box::new(PRepeat(false, Box::new(PCode(String::from("char(\'b\')")))))))
                    )
                ),
            Reject(_) =>
                assert_eq!(true, false)
        };
    }

    #[test]
    fn it_parse_a_repeatable_character_with_binding_with_choice() {
        let response = celma_language().parse(CharStream::new("a={char('a')}+ | b={char('b')}+"));

        match response {
            Success(ast, _, _) =>
                assert_eq!(
                    ast,
                    PChoice(
                        Box::new(PBind(String::from("a"), Box::new(PRepeat(false, Box::new(PCode(String::from("char(\'a\')"))))))),
                        Box::new(PBind(String::from("b"), Box::new(PRepeat(false, Box::new(PCode(String::from("char(\'b\')")))))))
                    )
                ),
            Reject(_) =>
                assert_eq!(true, false)
        };
    }

    #[test]
    fn it_parse_a_character_with_map() {
        let response = celma_language().parse(CharStream::new("a={char('a')} => { Result(a) }"));

        match response {
            Success(ast, _, _) =>
                assert_eq!(
                    ast,
                    PMap(Box::new(PBind(String::from("a"), Box::new(PCode(String::from("char(\'a\')"))))), String::from(" Result(a) "))
                ),
            Reject(_) =>
                assert_eq!(true, false)
        };
    }
}
