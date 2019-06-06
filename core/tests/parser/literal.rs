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
mod tests_literal {
    use celma_core::parser::literal::string;
    use celma_core::parser::or::OrOperation;
    use celma_core::parser::parser::Parse;
    use celma_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_a_str() {
        let response = string("hello").parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|v, _, _| v == "hello", |_, _| false), true);
    }

    #[test]
    fn it_parse_a_str_and_consume() {
        let response = string("hello").parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|_, _, b| b, |_, _| false), true);
    }

    #[test]
    fn it_cannot_parse_a_str() {
        let response = string("Hello").parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|_, _, _| false, |_, _| true), true);
    }

    #[test]
    fn it_cannot_parse_a_str_with_a_or() {
        let response = string("hek")
            .or(string("hello"))
            .parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|v, _, _| v == "hello", |_, _| false), true);
    }
}
