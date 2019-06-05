#[cfg(test)]
mod tests_and {
    use celma_core::parser::and::AndOperation;
    use celma_core::parser::and::AndProjection;
    use celma_core::parser::char::char;
    use celma_core::parser::parser::Parse;
    use celma_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_two_character() {
        let response = char('a').and(char('b')).parse(CharStream::new("ab"));

        assert_eq!(response.fold(|v, _, _| v == ('a', 'b'), |_| false), true);
    }

    #[test]
    fn it_parse_two_character_and_drop_right() {
        let response = char('a').and(char('b')).left().parse(CharStream::new("ab"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_| false), true);
    }

    #[test]
    fn it_parse_two_character_and_drop_left() {
        let response = char('a')
            .and(char('b'))
            .right()
            .parse(CharStream::new("ab"));

        assert_eq!(response.fold(|v, _, _| v == 'b', |_| false), true);
    }
}
