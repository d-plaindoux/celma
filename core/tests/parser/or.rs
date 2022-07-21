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
mod tests_or {
    use celma_core::parser::and::AndOperation;
    use celma_core::parser::and::AndProjection;
    use celma_core::parser::char::char;
    use celma_core::parser::or::OrOperation;
    use celma_core::parser::parser::Parse;
    use celma_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_one_character() {
        let response = char('a').or(char('b')).parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_, _| false), true);
    }

    #[test]
    fn it_parse_one_character_and_fails() {
        let response = (char('a').and(char('c')).left())
            .or(char('a'))
            .parse(CharStream::new("ab"));

        assert_eq!(response.fold(|_, _, _| false, |_, c| c), true);
    }
}
