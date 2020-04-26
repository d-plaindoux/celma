/*
   Copyright 2019-2020 Didier Plaindoux

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
    use celma_core::parser::char::{digit, space};
    use celma_core::parser::core::eos;
    use celma_core::parser::parser::Parse;
    use celma_core::parser::response::Response::Success;
    use celma_core::stream::char_stream::CharStream;
    use celma_macro::{parsec, parsec_rules};

    #[derive(Clone)]
    pub enum Expr {
        Number(f64),
        Plus(Box<Expr>, Box<Expr>),
        Mult(Box<Expr>, Box<Expr>),
    }

    impl Expr {
        fn eval(&self) -> f64 {
            match self {
                Expr::Number(f) => *f,
                Expr::Plus(l, r) => l.eval() + r.eval(),
                Expr::Mult(l, r) => l.eval() * r.eval(),
            }
        }
    }

    #[derive(Clone)]
    pub enum Operator {
        Plus,
        Mult,
    }

    fn mk_operation(l: Expr, r: Option<(Operator, Expr)>) -> Expr {
        match r {
            None => l,
            Some((Operator::Plus, r)) => Expr::Plus(Box::new(l), Box::new(r)),
            Some((Operator::Mult, r)) => Expr::Mult(Box::new(l), Box::new(r)),
        }
    }

    fn mk_string(a: Vec<char>) -> String {
        a.into_iter().collect::<String>()
    }

    fn mk_f64(a: Vec<char>) -> f64 {
        mk_string(a).parse().unwrap()
    }

    parsec_rules!(
        let expr:{Expr}   = (s=sexpr S e=(_=op S _=expr)?) -> {mk_operation(s,e)}
        let op:{Operator} = ('+' -> { Operator::Plus })
                          | ('*' -> { Operator::Mult })
        let sexpr:{Expr}  = ('(' S _=expr S ')') | number
        let number:{Expr} = f=NUMBER -> {Expr::Number(f)}
    );

    parsec_rules!(
        let NUMBER:{f64} = c=#(INT ('.' NAT)? (('E'|'e') INT)?) -> {mk_f64(c)}
        let INT:{()}     = ('-'|'+')? NAT                       -> {}
        let NAT:{()}     = digit+                               -> {}
        let S:{()}       = space*                               -> {}
    );

    #[test]
    fn it_parse_expr1() {
        let response = parsec!(_=expr {eos()}).parse(CharStream::new("1 + 2"));

        match response {
            Success(v, _, _) => assert_eq!(v.eval(), 3.0),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_parse_expr2() {
        let response = expr()
            .and_left(eos())
            .parse(CharStream::new("(1 + -2) * 4"));

        match response {
            Success(v, _, _) => assert_eq!(v.eval(), -4.0),
            _ => assert_eq!(true, false),
        }
    }
}
