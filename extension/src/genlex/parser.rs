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

use celma_core::parser::literal::{delimited_char, delimited_string};
use celma_core::parser::specs::{Combine, Parse};

use celma_core::stream::specs::Stream;

use crate::genlex::token::Token;

#[allow(dead_code)]
fn tokenizer<'a, S>(
    _operators: Vec<String>,
    _keywords: Vec<String>,
) -> impl Parse<Token, S> + Combine<Token> + 'a
where
    S: Stream<Item = char> + 'a,
{
    use celma_core::parser::char::{alpha, digit, space};
    use celma_macro::parsec_rules;

    fn mk_char(a: Vec<char>) -> char {
        *a.first().unwrap()
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
        let CHAR:{Token}     = c=delimited_char                     -> { Token::Char(c) }
        let STRING:{Token}   = c=delimited_string                   -> { Token::String(c) }
        let IDENT:{Token}    = i=#(alpha (alpha|digit|'_')*)        -> { Token::Ident(mk_string(i)) }
        let NUMBER:{Token}   = c=#(INT ('.' NAT)? (('E'|'e') INT)?) -> { Token::Float(mk_f64(c)) }
        // let OPERATOR:{Token} = TODO
    );

    token()
}
