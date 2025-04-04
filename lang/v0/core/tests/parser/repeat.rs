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
mod tests_repeat {
    use celma_v0_core::parser::char::a_char;
    use celma_v0_core::parser::repeat::RepeatOperation;
    use celma_v0_core::parser::specs::Parse;
    use celma_v0_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_zero_character() {
        let response = a_char('a').opt_rep().parse(CharStream::new(""));

        assert_eq!(response.fold(|v, _, _| v.is_empty(), |_, _| false), true);
    }

    #[test]
    fn it_cannot_parse_zero_character() {
        let response = a_char('a').rep().parse(CharStream::new(""));

        assert_eq!(response.fold(|_, _, _| false, |_, _| true), true);
    }

    #[test]
    fn it_parse_one_character() {
        let response = a_char('a').opt_rep().parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v.len() == 1, |_, _| false), true);
    }

    #[test]
    fn it_parse_three_characters() {
        let response = a_char('a').rep().parse(CharStream::new("aaab"));

        assert_eq!(response.fold(|v, _, _| v.len() == 3, |_, _| false), true);
    }
}
