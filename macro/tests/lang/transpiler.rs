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
mod tests_transpiler {
    use celma_core::parser::and::AndOperation;
    use celma_core::parser::core::eos;
    use celma_core::parser::specs::Parse;
    use celma_core::parser::response::Response::Success;
    use celma_core::stream::char_stream::CharStream;
    use celma_macro::parsec;

    #[test]
    fn it_parse_aaa() {
        let a = parsec!('a'*);

        let response = a.and_left(eos()).parse(CharStream::new("aaa"));

        match response {
            Success(v, _, _) => assert_eq!(v.len(), 3),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_baaa() {
        let a = parsec!('a'*);
        let b = parsec!('b' v={a});

        let response = b.and_left(eos()).parse(CharStream::new("baaa"));

        match response {
            Success(v, _, _) => assert_eq!(v.len(), 3),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_true() {
        let tf = parsec!(("true"  -> { true  }) | ("false" -> { false }));

        let response = tf.and_left(eos()).parse(CharStream::new("true"));

        match response {
            Success(v, _, _) => assert_eq!(v, true),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_false() {
        let tf = parsec!(("true"  -> { true  }) | ("false" -> { false }));

        let response = tf.and_left(eos()).parse(CharStream::new("false"));

        match response {
            Success(v, _, _) => assert_eq!(v, false),
            _ => assert_eq!(true, false),
        }
    }
}
