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
mod tests_parser_stream {
    use celma_core::parser::char::char_in_range;
    use celma_core::parser::core::any;
    use celma_core::parser::fmap::FMapOperation;
    use celma_core::parser::specs::Parse;
    use celma_core::parser::repeat::RepeatOperation;
    use celma_core::stream::char_stream::CharStream;
    use celma_core::stream::parser_stream::ParserStream;

    #[derive(Clone, Eq, PartialEq)]
    struct Item(char);

    #[test]
    fn it_parse_two_character() {
        let parser = char_in_range('a'..'z').fmap(|v| Item(v));
        let stream = ParserStream::new(&parser, CharStream::new("ab"));
        let response = any().rep().parse(stream);

        assert_eq!(
            response.fold(|v, _, _| v == vec!(Item('a'), Item('b')), |_, _| false),
            true
        );
    }
}
