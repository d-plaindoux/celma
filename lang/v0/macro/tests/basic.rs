/*
 * Copyright 2019-2025 Didier Plaindoux
=======
 * Copyright 2019-2021 Didier Plaindoux
>>>>>>> 45ec19c (Manage compiler warnings and change License header)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[cfg(test)]
mod tests_transpiler {
    use celma_core::parser::and::AndOperation;
    use celma_core::parser::core::eos;
    use celma_core::parser::response::Response::Success;
    use celma_core::parser::specs::Parse;
    use celma_core::stream::char_stream::CharStream;
    use celma_lang_v0_macro::parsec_rules;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Expr {
        Text(String),
        Var(String),
        Seq(Vec<Expr>),
    }

    fn mk_string(a: Vec<char>) -> String {
        a.into_iter().collect::<String>()
    }

    parsec_rules!(
        let bash:{Expr} = s=(text | var)*    -> {Expr::Seq(s)}
        let text:{Expr} = t=^"${"+           -> {Expr::Text(mk_string(t))}
        let var:{Expr}  = ("${" v=^'}'* '}') -> {Expr::Var(mk_string(v))}
    );

    #[test]
    fn it_parse_a_text() {
        let response = bash().and_left(eos()).parse(CharStream::new("Hello"));

        match response {
            Success(v, _, _) => assert_eq!(v, Expr::Seq(vec!(Expr::Text("Hello".to_owned())))),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_a_var() {
        let response = bash().and_left(eos()).parse(CharStream::new("${world}"));

        match response {
            Success(v, _, _) => assert_eq!(v, Expr::Seq(vec!(Expr::Var("world".to_owned())))),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_a_seq() {
        let response = bash()
            .and_left(eos())
            .parse(CharStream::new("Hello ${world}"));

        match response {
            Success(v, _, _) => assert_eq!(
                v,
                Expr::Seq(vec!(
                    Expr::Text("Hello ".to_owned()),
                    Expr::Var("world".to_owned())
                ))
            ),
            _ => assert_eq!(true, false),
        }
    }
}
