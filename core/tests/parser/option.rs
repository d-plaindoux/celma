#![allow(dead_code)]

#[cfg(test)]
mod tests_option {
    use celma_core::parser::char::char;
    use celma_core::parser::option::OptionalOperation;
    use celma_core::parser::parser::Parse;
    use celma_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_zero_character() {
        let response = char('a').opt().parse(CharStream::new(""));

        assert_eq!(response.fold(|v, _, _| v == None, |_, _| false), true);
    }

    #[test]
    fn it_parse_one_character() {
        let response = char('a').opt().parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == Some('a'), |_, _| true), true);
    }
}
