#![allow(dead_code)]

#[cfg(test)]
mod tests_char {
    use celma::parser::char::alpha;
    use celma::parser::char::alpha_lower;
    use celma::parser::char::alpha_upper;
    use celma::parser::char::char;
    use celma::parser::char::digit;
    use celma::parser::char::not_char;
    use celma::parser::parser::Parse;
    use celma::stream::char_stream::CharStream;

    #[test]
    fn it_parse_a_specific_character() {
        let response = char('a').parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_| false), true);
    }

    #[test]
    fn it_cannot_parse_a_specific_character() {
        let response = char('a').parse(CharStream::new("b"));

        assert_eq!(response.fold(|_, _, _| false, |_| true), true);
    }

    #[test]
    fn it_parse_another_specific_character() {
        let response = not_char('b').parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_| false), true);
    }

    #[test]
    fn it_cannot_parse_another_specific_character() {
        let response = not_char('a').parse(CharStream::new("a"));

        assert_eq!(response.fold(|_, _, _| false, |_| true), true);
    }

    #[test]
    fn it_can_parse_an_integer() {
        let response = digit().parse(CharStream::new("1"));

        assert_eq!(response.fold(|v, _, _| v == '1', |_| false), true);
    }

    #[test]
    fn it_cannot_parse_an_integer() {
        let response = digit().parse(CharStream::new("a"));

        assert_eq!(response.fold(|_, _, _| false, |_| true), true);
    }

    #[test]
    fn it_can_parse_a_lowercase_alpha() {
        let response = alpha_lower().parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_| false), true);
    }

    #[test]
    fn it_cannot_parse_a_lowercase_alpha() {
        let response = alpha_lower().parse(CharStream::new("A"));

        assert_eq!(response.fold(|_, _, _| false, |_| true), true);
    }

    #[test]
    fn it_can_parse_a_uppercase_alpha() {
        let response = alpha_upper().parse(CharStream::new("A"));

        assert_eq!(response.fold(|v, _, _| v == 'A', |_| false), true);
    }

    #[test]
    fn it_cannot_parse_a_uppercase_alpha() {
        let response = alpha_upper().parse(CharStream::new("a"));

        assert_eq!(response.fold(|_, _, _| false, |_| true), true);
    }

    #[test]
    fn it_can_parse_an_alpha() {
        let response = alpha().parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_| false), true);
    }

    #[test]
    fn it_can_parse_another_alpha() {
        let response = alpha().parse(CharStream::new("A"));

        assert_eq!(response.fold(|v, _, _| v == 'A', |_| false), true);
    }

    #[test]
    fn it_cannot_parse_an_alpha() {
        let response = alpha().parse(CharStream::new("0"));

        assert_eq!(response.fold(|_, _, _| false, |_| true), true);
    }
}

// -------------------------------------------------------------------------------------------------
