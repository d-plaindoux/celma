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
mod tests_and {
    use celma_core::parser::parser::Parse;
    use celma_core::parser::response::Response::Success;
    use celma_core::stream::char_stream::CharStream;
    use celma_lang::meta::parser::celma_parsec_rules;
    use celma_lang::meta::transpiler::Transpile;

    #[test]
    fn it_parse_one_char_rule() {
        let response = celma_parsec_rules()
            .parse(CharStream::new("let a:{char} = {char('a')}"))
            .fmap(|ast| ast.transpile());

        match response {
            Success(_, _, _) => assert_eq!(true, true), // TODO
            _ => assert_eq!(true, false),
        };
    }
}