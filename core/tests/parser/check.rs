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
mod tests_check {
    use celma_core::parser::and::AndOperation;
    use celma_core::parser::char::a_char;
    use celma_core::parser::check::check;
    use celma_core::parser::specs::Parse;
    use celma_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_two_character() {
        let response = check(a_char('a').and(a_char('b'))).parse(CharStream::new("ab"));

        assert_eq!(
            response.fold(|v, _, _| v == vec!['a', 'b'], |_, _| false),
            true
        );
    }
}
