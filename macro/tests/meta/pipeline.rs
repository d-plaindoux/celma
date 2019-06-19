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
    use celma_core::parser::char::{digit, space};
    use celma_core::parser::core::eos;
    use celma_core::parser::fmap::FMapOperation;
    use celma_core::parser::parser::{Combine, Parse};
    use celma_core::parser::response::Response::Success;
    use celma_core::parser::satisfy::Satisfy;
    use celma_core::stream::char_stream::CharStream;
    use celma_core::stream::parser_stream::ParserStream;
    use celma_core::stream::stream::Stream;
    use celma_macro::parsec_rules;

    // ---------------------------------------------------------------------------------------------
    // Tokens
    // ---------------------------------------------------------------------------------------------

    #[derive(Clone, Eq, PartialEq)]
    pub enum Token {
        Int(i64),
        Keyword(char),
    }

    // ---------------------------------------------------------------------------------------------
    // Basic parsers
    // ---------------------------------------------------------------------------------------------

    fn kint<'a, S: 'a>() -> impl Parse<i64, S> + Combine<i64> + 'a
    where
        S: Stream<Item = Token>,
    {
        Satisfy::new(|v| match v {
            Token::Int(_) => true,
            _ => false,
        })
        .fmap(|v| match v {
            Token::Int(i) => i,
            _ => panic!(),
        })
    }

    fn kwd<'a, S: 'a>(k: char) -> impl Parse<char, S> + Combine<char> + 'a
    where
        S: Stream<Item = Token>,
    {
        Satisfy::new(move |v| match v {
            Token::Keyword(s) => *s == k,
            _ => false,
        })
        .fmap(|v| match v {
            Token::Keyword(s) => s.clone(),
            _ => panic!(),
        })
    }

    // ---------------------------------------------------------------------------------------------
    // Expressions
    // ---------------------------------------------------------------------------------------------

    #[derive(Clone)]
    pub enum Expr {
        Number(i64),
        Plus(Box<Expr>, Box<Expr>),
        Mult(Box<Expr>, Box<Expr>),
    }

    impl Expr {
        fn eval(&self) -> i64 {
            match self {
                Expr::Number(f) => *f,
                Expr::Plus(l, r) => l.eval() + r.eval(),
                Expr::Mult(l, r) => l.eval() * r.eval(),
            }
        }
    }

    // ---------------------------------------------------------------------------------------------
    // Functions dedicated to parsers
    // ---------------------------------------------------------------------------------------------

    #[derive(Clone)]
    pub enum Operator {
        Plus,
        Mult,
    }

    fn mk_string(a: Vec<char>) -> String {
        a.into_iter().collect::<String>()
    }

    fn mk_operation(l: Expr, r: Option<(Operator, Expr)>) -> Expr {
        match r {
            None => l,
            Some((Operator::Plus, r)) => Expr::Plus(Box::new(l), Box::new(r)),
            Some((Operator::Mult, r)) => Expr::Mult(Box::new(l), Box::new(r)),
        }
    }

    // ---------------------------------------------------------------------------------------------
    // Tokenizer
    // ---------------------------------------------------------------------------------------------

    parsec_rules!(
        let token{char}:{Token}   = S _=(float|keyword) S
        let float{char}:{Token}   = f=!(INT)             -> { Token::Int(f)   }
        let keyword{char}:{Token} = s=('+'|'*'|'('|')')    -> { Token::Keyword(s) }

        let INT{char}:{i64}       = c=#(('-'|'+')? NAT)    -> { mk_string(c).parse().unwrap() }
        let NAT{char}:{()}        = digit+                 -> {}
        let S{char}:{()}          = space*                 -> {}
    );

    // ---------------------------------------------------------------------------------------------
    // Expression parser
    // ---------------------------------------------------------------------------------------------

    // Lexemes
    parsec_rules!(
        let PLUS{Token}:{()}   = {kwd('+')} -> {}
        let MULT{Token}:{()}   = {kwd('*')} -> {}
        let LPAREN{Token}:{()} = {kwd('(')} -> {}
        let RPAREN{Token}:{()} = {kwd(')')} -> {}
    );

    // Parser
    parsec_rules!(
        let expr{Token}:{Expr}   = (s=sexpr e=(_=op _=expr)?) -> {mk_operation(s,e)}
        let op{Token}:{Operator} = (PLUS                      -> { Operator::Plus })
                                 | (MULT                      -> { Operator::Mult })
        let sexpr{Token}:{Expr}  = (LPAREN _=expr RPAREN)
                                 | number
        let number{Token}:{Expr} = i=kint                     -> {Expr::Number(i)}
    );

    // ---------------------------------------------------------------------------------------------

    #[test]
    fn it_parse_expr1() {
        let tokenizer = token();
        let stream = ParserStream::new(&tokenizer, CharStream::new("1 + 2"));
        let response = expr().and_left(eos()).parse(stream);

        match response {
            Success(v, _, _) => assert_eq!(v.eval(), 3),
            _ => assert_eq!(true, false),
        }
    }
}
