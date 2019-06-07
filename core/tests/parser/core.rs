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

#[cfg(test)]
mod tests_core {
    use celma_core::parser::core::any;
    use celma_core::parser::core::eos;
    use celma_core::parser::core::fail;
    use celma_core::parser::core::returns;
    use celma_core::parser::parser::Parse;
    use celma_core::stream::char_stream::CharStream;

    #[test]
    fn it_parse_and_returns_unit() {
        let response = returns(()).parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == (), |_, _| false), true);
    }

    #[test]
    fn it_parse_and_fails_on_demand() {
        let response = fail::<()>(true).parse(CharStream::new("a"));

        assert_eq!(response.fold(|_, _, _| false, |_, v| v), true);
    }

    #[test]
    fn it_parse_any_character() {
        let response = any().parse(CharStream::new("a"));

        assert_eq!(response.fold(|v, _, _| v == 'a', |_, _| false), true);
    }

    #[test]
    fn it_cannot_parse_any_character() {
        let response = any().parse(CharStream::new(""));

        assert_eq!(response.fold(|_, _, _| false, |_, _| true), true);
    }

    #[test]
    fn it_checks_eos() {
        let response = eos().parse(CharStream::new(""));

        assert_eq!(response.fold(|_, _, _| true, |_, _| false), true);
    }

    #[test]
    fn it_cannot_check_eos() {
        let response = eos().parse(CharStream::new("a"));

        assert_eq!(response.fold(|_, _, _| false, |_, _| true), true);
    }
}
