#![allow(dead_code)]

#[cfg(test)]
mod tests_literal {
    use celma::parser::literal::string;
    use celma::parser::or::OrOperation;
    use celma::parser::parser::Parse;
    use celma::stream::char_stream::CharStream;

    #[test]
    fn it_parse_a_str() {
        let response = string("hello").parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|v, _, _| v == "hello", |_| false), true);
    }

    #[test]
    fn it_parse_a_str_and_consume() {
        let response = string("hello").parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|_, _, b| b, |_| false), true);
    }

    #[test]
    fn it_cannot_parse_a_str() {
        let response = string("Hello").parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|_, _, _| false, |_| true), true);
    }

    #[test]
    fn it_cannot_parse_a_str_with_a_or() {
        let response = string("hek")
            .or(string("hello"))
            .parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|v, _, _| v == "hello", |_| false), true);
    }
}
