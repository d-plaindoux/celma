#![allow(dead_code)]

#[cfg(test)]
mod tests_lazy {
    use celma_core::parser::char::char;
    use celma_core::parser::lazy::lazy;
    use celma_core::parser::parser::Parse;
    use celma_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_a_specific_character() {
        let response = lazy(|| char('a')).parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_| false), true);
    }
}

// -------------------------------------------------------------------------------------------------
