#![allow(dead_code)]

#[cfg(test)]
mod tests_repeat {
    use celma::parser::char::char;
    use celma::parser::parser::Parse;
    use celma::parser::repeat::RepeatOperation;
    use celma::stream::char_stream::CharStream;

    #[test]
    fn it_parse_zero_character() {
        let response = char('a').opt_rep().parse(CharStream::new(""));

        assert_eq!(response.fold(|v, _, _| v.is_empty(), |_| false), true);
    }

    #[test]
    fn it_cannot_parse_zero_character() {
        let response = char('a').rep().parse(CharStream::new(""));

        assert_eq!(response.fold(|_, _, _| false, |_| true), true);
    }

    #[test]
    fn it_parse_one_character() {
        let response = char('a').opt_rep().parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v.len() == 1, |_| false), true);
    }

    #[test]
    fn it_parse_three_characters() {
        let response = char('a').rep().parse(CharStream::new("aaab"));

        assert_eq!(response.fold(|v, _, _| v.len() == 3, |_| false), true);
    }
}
