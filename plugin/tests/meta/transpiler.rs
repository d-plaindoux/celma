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
mod tests_transpiler {
    use celma_core::parser::and::AndOperation;
    use celma_core::parser::core::eos;
    use celma_core::parser::parser::Parse;
    use celma_core::parser::response::Response::Success;
    use celma_core::stream::char_stream::CharStream;
    use celma_plugin::parsec_rules;

    #[test]
    fn it_parse_aaa() {
        parsec_rules!(
            let a:{Vec<char>} = 'a'*
        );

        let response = a().and_left(eos()).parse(CharStream::new("aaa"));

        match response {
            Success(v, _, _) => assert_eq!(v.len(), 3),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_baaa() {
        parsec_rules!(
            let a:{Vec<char>} = 'a'*
            let b:{String}    = 'b' v=a -> { v.into_iter().collect() }
        );

        let response = b().and_left(eos()).parse(CharStream::new("baaa"));

        match response {
            Success(v, _, _) => assert_eq!(v.len(), 3),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_true() {
        parsec_rules!(
            let tf:{bool} = ("true"  -> { true  })
                          | ("false" -> { false })
        );

        let response = tf().and_left(eos()).parse(CharStream::new("true"));

        match response {
            Success(v, _, _) => assert_eq!(v, true),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_false() {
        parsec_rules!(
            let tf:{bool} = ("true"  -> { true  })
                          | ("false" -> { false })
        );

        let response = tf().and_left(eos()).parse(CharStream::new("false"));

        match response {
            Success(v, _, _) => assert_eq!(v, false),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_1_true_and_reverse() {
        // TODO(didier) Review the syntax in order to remove the uppermost parenthesis
        parsec_rules!(
            let ib:{(bool,u32)} = (a=('1' -> { 1 }) ',' b=("true" -> { true })) -> { (b, a) }
        );

        let response = ib().and_left(eos()).parse(CharStream::new("1,true"));

        match response {
            Success(v, _, _) => assert_eq!(v, (true, 1)),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_with_recursive_parser() {
        parsec_rules!(
            let parens:{()} = ('(' parens ')')? -> { () }
        );

        let response = parens()
            .and_left(eos())
            .parse(CharStream::new("((((((((()))))))))"));

        match response {
            Success(_, _, _) => assert_eq!(true, true),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_a_string_elem() {
        parsec_rules!(
            let parens:{()} = (^'"'|"\\\"" -> { '"' }) -> { () }
        );

        let response = parens().and_left(eos()).parse(CharStream::new("a"));

        match response {
            Success(_, _, _) => assert_eq!(true, true),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_a_special_string_elem() {
        parsec_rules!(
            let parens:{char} = ("\"" -> { '"' }) | ^'"'
        );

        let response = parens().and_left(eos()).parse(CharStream::new("\""));

        match response {
            Success(_, _, _) => assert_eq!(true, true),
            _ => assert_eq!(true, false),
        }
    }
}
