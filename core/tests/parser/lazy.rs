#![allow(dead_code)]

#[cfg(test)]
mod tests_lazy {
    use celma::parser::char::char;
    use celma::parser::lazy::lazy;
    use celma::parser::parser::Parse;
    use celma::stream::char_stream::CharStream;

    #[test]
    fn it_parse_a_specific_character() {
        let response = lazy(|| char('a')).parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_| false), true);
    }
}

// -------------------------------------------------------------------------------------------------
