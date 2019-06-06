#![allow(dead_code)]

#[cfg(test)]
mod tests_monadic {
    use celma_core::parser::and::AndOperation;
    use celma_core::parser::bind::BindOperation;
    use celma_core::parser::char::char;
    use celma_core::parser::core::eos;
    use celma_core::parser::fmap::FMapOperation;
    use celma_core::parser::literal::string;
    use celma_core::parser::parser::Parse;
    use celma_core::parser::repeat::RepeatOperation;
    use celma_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_a_str_and_fmap_it_to_u32() {
        let response = string("hello")
            .fmap(|a| a.len())
            .parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|v, _, _| v == 5, |_, _| false), true);
    }

    #[test]
    fn it_parse_a_str_and_bind_it_a_str_parser() {
        let response = string("he")
            .bind(|a| char(a.chars().next().unwrap()).rep().and(eos()))
            .parse(CharStream::new("hehhhhhhh"));

        assert_eq!(response.fold(|v, _, _| v.0.len() == 7, |_, _| false), true);
    }
}
