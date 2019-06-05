#![allow(dead_code)]

#[cfg(test)]
mod tests_or {
    use celma_core::parser::and::AndOperation;
    use celma_core::parser::and::AndProjection;
    use celma_core::parser::char::char;
    use celma_core::parser::or::OrOperation;
    use celma_core::parser::parser::Parse;
    use celma_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_one_character() {
        let response = char('a').or(char('b')).parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_| false), true);
    }

    #[test]
    fn it_parse_one_character_and_fails() {
        let response = (char('a').and(char('c')).left())
            .or(char('a'))
            .parse(CharStream::new("ab"));

        assert_eq!(response.fold(|_, _, _| false, |c| c), true);
    }
}
