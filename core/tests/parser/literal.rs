/*
   Copyright 2019-2025 Didier Plaindoux

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

#[cfg(test)]
mod tests_literal {
    use celma_core::parser::literal::{delimited_string, string, delimited_char};
    use celma_core::parser::or::OrOperation;
    use celma_core::parser::parser::Parse;
    use celma_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_a_string() {
        let response = string("hello").parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|v, _, _| v == "hello", |_, _| false), true);
    }

    #[test]
    fn it_parse_a_string_and_consume() {
        let response = string("hello").parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|_, _, b| b, |_, _| false), true);
    }

    #[test]
    fn it_parse_a_string_with_non_escaped_char() {
        let response = string("\"").parse(CharStream::new(r#"""#));

        assert_eq!(response.fold(|v, _, _| v, |_, _| ""), "\"");
    }

    #[test]
    fn it_parse_a_string_with_escaped_char() {
        let response = string(r#"\""#).parse(CharStream::new(r#"\""#));

        assert_eq!(response.fold(|v, _, _| v, |_, _| ""), "\\\""); // ???
    }

    #[test]
    fn it_cannot_parse_a_string() {
        let response = string("Hello").parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|_, _, _| false, |_, _| true), true);
    }

    #[test]
    fn it_cannot_parse_a_string_with_a_or() {
        let response = string("hek")
            .or(string("hello"))
            .parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|v, _, _| v == "hello", |_, _| false), true);
    }

    #[test]
    fn it_parse_a_delimited_string() {
        let response = delimited_string().parse(CharStream::new(r#""hello""#));

        assert_eq!(response.fold(|v, _, _| v == "hello", |_, _| false), true);
    }

    #[test]
    fn it_parse_a_delimited_string_with_an_escaped_double_quote() {
        let response = delimited_string().parse(CharStream::new(r#""hel\"lo""#));

        assert_eq!(response.fold(|v, _, _| v == "hel\"lo", |_, _| false), true);
    }

    #[test]
    fn it_parse_a_delimited_string_with_an_escaped_carriage_return() {
        let response = delimited_string().parse(CharStream::new(r#""hel\nlo""#));

        assert_eq!(response.fold(|v, _, _| v == "hel\nlo", |_, _| false), true);
    }

    #[test]
    fn it_parse_a_delimited_char() {
        let response = delimited_char().parse(CharStream::new("'a'"));

        assert_eq!(response.fold(|v, _, _| v, |_, _| '\0'), 'a');
    }

    #[test]
    fn it_parse_a_delimited_char_escaped_carriage_return() {
        let response = delimited_char().parse(CharStream::new(r#"'\n'"#));

        assert_eq!(response.fold(|v, _, _| v, |_, _| '\0'), '\n');
    }

    #[test]
    fn it_parse_a_delimited_char_escaped_slash() {
        let response = delimited_char().parse(CharStream::new(r#"'\\'"#));

        assert_eq!(response.fold(|v, _, _| v, |_, _| '\0'), '\\');
    }

    #[test]
    fn it_parse_a_delimited_char_escaped_quote() {
        let response = delimited_char().parse(CharStream::new(r#"'\''"#));

        assert_eq!(response.fold(|v, _, _| v, |_, _| '\0'), '\'');
    }
}
