/*
   Copyright 2019-2021 Didier Plaindoux

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
mod tests_location {
    use celma_v0_core::parser::char::a_char;
    use celma_v0_core::parser::location::locate;
    use celma_v0_core::parser::specs::Parse;
    use celma_v0_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_one_character() {
        let response = locate(a_char('a')).parse(CharStream::new("a"));

        assert_eq!(
            response.fold(|v, _, _| v.value == ('a'), |_, _| false),
            true
        );
    }

    #[test]
    fn it_parse_one_character_with_right_start_location() {
        let response = locate(a_char('a')).parse(CharStream::new("a"));

        assert_eq!(
            response.fold(|v, _, _| v.start, |_, _| (0, 0, 0)),
            (0, 1, 0)
        );
    }

    #[test]
    fn it_parse_one_character_with_right_end_location() {
        let response = locate(a_char('a')).parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v.end, |_, _| (0, 0, 0)), (1, 1, 1));
    }
}
