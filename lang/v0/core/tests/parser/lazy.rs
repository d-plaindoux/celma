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
mod tests_lazy {
    use celma_v0_core::parser::char::a_char;
    use celma_v0_core::parser::lazy::lazy;
    use celma_v0_core::parser::specs::Parse;
    use celma_v0_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_a_specific_character() {
        let response = lazy(|| a_char('a')).parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_, _| false), true);
    }
}

// -------------------------------------------------------------------------------------------------
