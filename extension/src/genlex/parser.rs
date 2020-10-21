/*
   Copyright 2019-2023 Didier Plaindoux

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

use celma_core::parser::parser::{Combine, Parse};
use celma_core::parser::literal::{delimited_string, delimited_char};

use celma_core::stream::stream::Stream;

use crate::genlex::token::Token;

fn tokenizer<'a, S: 'a>(
    operators: Vec<String>,
    keywords: Vec<String>,
) -> impl Parse<Token, S> + Combine<Token> + 'a
    where
        S: Stream<Item=char>,
{
    use celma_core::parser::char::{alpha, digit, space};
    use celma_macro::parsec_rules;

    fn mk_char(a: Vec<char>) -> char {
        a.first().unwrap().clone()
    }

    fn mk_string(a: Vec<char>) -> String {
        a.into_iter().collect::<String>()
    }

    fn mk_f64(a: Vec<char>) -> f64 {
        mk_string(a).parse().unwrap()
    }

    parsec_rules!(
        let INT:{()}    = ('-'|'+')? NAT -> {}
        let NAT:{()}    = digit+         -> {}
        let SPACES:{()} = space*         -> {}
    );

    parsec_rules!(
        let token:{Token}    = SPACES _=(STRING|IDENT|NUMBER) SPACES
        let CHAR:{Token}     = c={delimited_char()}                 -> { Token::Char(c) }
        let STRING:{Token}   = c={delimited_string()}               -> { Token::String(c) }
        let IDENT:{Token}    = i=#(alpha (alpha|digit|'_')*)        -> { Token::Ident(mk_string(i)) }
        let NUMBER:{Token}   = c=#(INT ('.' NAT)? (('E'|'e') INT)?) -> { Token::Float(mk_f64(c)) }
        // let OPERATOR:{Token} = TODO
    );

    token()
}
