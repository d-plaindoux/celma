#![allow(dead_code)]

#[cfg(test)]
mod tests_option {
    use celma::parser::char::char;
    use celma::parser::option::OptionalOperation;
    use celma::parser::parser::Parse;
    use celma::stream::char_stream::CharStream;

    #[test]
    fn it_parse_zero_character() {
        let response = char('a').opt().parse(CharStream::new(""));

        assert_eq!(response.fold(|v, _, _| v == None, |_| false), true);
    }

    #[test]
    fn it_parse_one_character() {
        let response = char('a').opt().parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == Some('a'), |_| true), true);
    }
}
