/*
   Copyright 2019 Didier Plaindoux

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

#![allow(dead_code)]

#[cfg(test)]
mod tests_char {
    use celma_core::parser::char::alpha;
    use celma_core::parser::char::alpha_lower;
    use celma_core::parser::char::alpha_upper;
    use celma_core::parser::char::char;
    use celma_core::parser::char::digit;
    use celma_core::parser::char::not_char;
    use celma_core::parser::parser::Parse;
    use celma_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_a_specific_character() {
        let response = char('a').parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_, _| false), true);
    }

    #[test]
    fn it_cannot_parse_a_specific_character() {
        let response = char('a').parse(CharStream::new("b"));

        assert_eq!(response.fold(|_, _, _| false, |_, _| true), true);
    }

    #[test]
    fn it_parse_another_specific_character() {
        let response = not_char('b').parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_, _| false), true);
    }

    #[test]
    fn it_cannot_parse_another_specific_character() {
        let response = not_char('a').parse(CharStream::new("a"));

        assert_eq!(response.fold(|_, _, _| false, |_, _| true), true);
    }

    #[test]
    fn it_can_parse_an_integer() {
        let response = digit().parse(CharStream::new("1"));

        assert_eq!(response.fold(|v, _, _| v == '1', |_, _| false), true);
    }

    #[test]
    fn it_cannot_parse_an_integer() {
        let response = digit().parse(CharStream::new("a"));

        assert_eq!(response.fold(|_, _, _| false, |_, _| true), true);
    }

    #[test]
    fn it_can_parse_a_lowercase_alpha() {
        let response = alpha_lower().parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_, _| false), true);
    }

    #[test]
    fn it_cannot_parse_a_lowercase_alpha() {
        let response = alpha_lower().parse(CharStream::new("A"));

        assert_eq!(response.fold(|_, _, _| false, |_, _| true), true);
    }

    #[test]
    fn it_can_parse_a_uppercase_alpha() {
        let response = alpha_upper().parse(CharStream::new("A"));

        assert_eq!(response.fold(|v, _, _| v == 'A', |_, _| false), true);
    }

    #[test]
    fn it_cannot_parse_a_uppercase_alpha() {
        let response = alpha_upper().parse(CharStream::new("a"));

        assert_eq!(response.fold(|_, _, _| false, |_, _| true), true);
    }

    #[test]
    fn it_can_parse_an_alpha() {
        let response = alpha().parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_, _| false), true);
    }

    #[test]
    fn it_can_parse_another_alpha() {
        let response = alpha().parse(CharStream::new("A"));

        assert_eq!(response.fold(|v, _, _| v == 'A', |_, _| false), true);
    }

    #[test]
    fn it_cannot_parse_an_alpha() {
        let response = alpha().parse(CharStream::new("0"));

        assert_eq!(response.fold(|_, _, _| false, |_, _| true), true);
    }
}

// -------------------------------------------------------------------------------------------------
