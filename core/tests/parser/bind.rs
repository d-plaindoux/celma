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
mod tests_monadic {
    use celma_core::parser::and::AndOperation;
    use celma_core::parser::bind::BindOperation;
    use celma_core::parser::char::a_char;
    use celma_core::parser::core::eos;
    use celma_core::parser::fmap::FMapOperation;
    use celma_core::parser::literal::string;
    use celma_core::parser::specs::Parse;
    use celma_core::parser::repeat::RepeatOperation;
    use celma_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_a_str_and_fmap_it_to_u32() {
        let response = string("hello")
            .fmap(|a| a.len())
            .parse(CharStream::new("hello world!"));

        assert_eq!(response.fold(|v, _, _| v == 5, |_, _| false), true);
    }

    #[test]
    fn it_parse_a_str_and_bind_it_a_str_parser() {
        let response = string("he")
            .bind(|a| a_char(a.chars().next().unwrap()).rep().and(eos()))
            .parse(CharStream::new("hehhhhhhh"));

        assert_eq!(response.fold(|v, _, _| v.0.len() == 7, |_, _| false), true);
    }
}
