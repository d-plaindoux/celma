#![allow(dead_code)]

#[cfg(test)]
mod tests_core {
    use celma::parser::core::any;
    use celma::parser::core::eos;
    use celma::parser::core::fail;
    use celma::parser::core::returns;
    use celma::parser::parser::Parse;
    use celma::stream::char_stream::CharStream;

    #[test]
    fn it_parse_and_returns_unit() {
        let response = returns(()).parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == (), |_| false), true);
    }

    #[test]
    fn it_parse_and_fails_on_demand() {
        let response = fail::<()>(true).parse(CharStream::new("a"));

        assert_eq!(response.fold(|_, _, _| false, |v| v), true);
    }

    #[test]
    fn it_parse_any_character() {
        let response = any().parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_| false), true);
    }

    #[test]
    fn it_cannot_parse_any_character() {
        let response = any().parse(CharStream::new(""));

        assert_eq!(response.fold(|_, _, _| false, |_| true), true);
    }

    #[test]
    fn it_checks_eos() {
        let response = eos().parse(CharStream::new(""));

        assert_eq!(response.fold(|_, _, _| true, |_| false), true);
    }

    #[test]
    fn it_cannot_check_eos() {
        let response = eos().parse(CharStream::new("a"));

        assert_eq!(response.fold(|_, _, _| false, |_| true), true);
    }
}
