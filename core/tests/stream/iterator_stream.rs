#[cfg(test)]
mod tests_parser_stream {
    use celma::parser::core::any;
    use celma::parser::parser::Parse;
    use celma::parser::repeat::RepeatOperation;
    use celma::stream::iterator_stream::IteratorStream;

    #[derive(Clone, Eq, PartialEq)]
    struct Item(char);

    #[test]
    fn it_parse_two_character() {
        let source = "abc".to_string();
        let stream = IteratorStream::new(source.chars());
        let response = any().rep().parse(stream);

        assert_eq!(
            response.fold(|v, _, _| v == vec!('a', 'b', 'c'), |_| false),
            true
        );
    }
}