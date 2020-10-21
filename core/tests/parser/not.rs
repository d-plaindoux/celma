/*
   Copyright 2019-2020 Didier Plaindoux

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
    use celma_core::parser::char::char;
    use celma_core::parser::literal::string;
    use celma_core::parser::not::NotOperation;
    use celma_core::parser::parser::Parse;
    use celma_core::parser::repeat::RepeatOperation;
    use celma_core::stream::char_stream::CharStream;
    use celma_core::parser::fmap::FMapOperation;

    #[test]
    fn it_parse_any_char_else_char_b() {
        let response = char('b').not().parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_, _| false), true);
    }

    #[test]
    fn it_parse_any_char_else_string_b() {
        let response = string("b").not().parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_, _| false), true);
    }

    #[test]
    fn it_parse_any_chars_else_string_b() {
        let response =
            string("de").not().rep()
                .fmap(|v| v.iter().collect::<String>())
                .parse(CharStream::new("abcdcde"));

        assert_eq!(response.fold(|v, _, _| v == "abcdc".to_owned(), |_, _| false), true);
    }
}
