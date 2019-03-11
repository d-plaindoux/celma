#![allow(dead_code)]

#[cfg(test)]
mod tests_monadic {
    use celma::parser::and::AndOperation;
    use celma::parser::char::char;
    use celma::parser::core::eos;
    use celma::parser::literal::string;
    use celma::parser::monadic::BindOperation;
    use celma::parser::monadic::FMapOperation;
    use celma::parser::parser::Parse;
    use celma::parser::repeat::RepeatOperation;
    use celma::stream::char_stream::CharStream;

    #[test]
    fn it_parse_a_str_and_fmap_it_to_u32() {
        let response = string("hello")
            .fmap(|a| a.len())
            .parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|v, _, _| v == 5, |_| false), true);
    }

    #[test]
    fn it_parse_a_str_and_bind_it_a_str_parser() {
        let response = string("he")
            .bind(|a| char(a.chars().next().unwrap()).rep().and(eos()))
            .parse(CharStream::new("hehhhhhhh"));

        assert_eq!(response.fold(|v, _, _| v.0.len() == 7, |_| false), true);
    }
}
