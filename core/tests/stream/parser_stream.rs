#[cfg(test)]
mod tests_parser_stream {
    use celma::parser::char::char_in_range;
    use celma::parser::core::any;
    use celma::parser::fmap::FMapOperation;
    use celma::parser::parser::Parse;
    use celma::parser::repeat::RepeatOperation;
    use celma::stream::char_stream::CharStream;
    use celma::stream::parser_stream::ParserStream;

    #[derive(Clone, Eq, PartialEq)]
    struct Item(char);

    #[test]
    fn it_parse_two_character() {
        let parser = char_in_range('a'..'z').fmap(|v| Item(v));
        let stream = ParserStream::new(&parser, CharStream::new("ab"));
        let response = any().rep().parse(stream);

        assert_eq!(
            response.fold(|v, _, _| v == vec!(Item('a'), Item('b')), |_| false),
            true
        );
    }
}
