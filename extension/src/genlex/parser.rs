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

use crate::genlex::token::Token;
use celma_core::parser::parser::{Combine, Parse};
use celma_core::stream::stream::Stream;

fn tokenizer<'a, S: 'a>(operators: Vec<String>) -> impl Parse<Token, S> + Combine<Token> + 'a
where
    S: Stream<Item = char>,
{
    use celma_core::parser::char::{alpha, digit, space};
    use celma_macro::parsec_rules;

    fn mk_char(a: &str) -> char {
        String::from(a).chars().next().unwrap()
    }

    fn mk_string(a: Vec<char>) -> String {
        a.into_iter().collect::<String>()
    }

    fn mk_f64(a: Vec<char>) -> f64 {
        mk_string(a).parse().unwrap()
    }

    parsec_rules!(
        let token:{Token}  = SPACES _=(STRING|IDENT|NUMBER) SPACES
        // let CHAR:{Token}   = ("'" c=(("\'"  -> {"\'"})|^"'")  "'")   -> { Token::Char(mk_char(c)) }
        let STRING:{Token} = ('"' c=#((("\"" -> {'\"'})|^'"')*) '"') -> { Token::String(mk_string(c)) }
        let IDENT:{Token}  = i=#(alpha (alpha|digit|'_')*)           -> { Token::Ident(mk_string(i)) }
        let NUMBER:{Token} = c=#(INT ('.' NAT)? (('E'|'e') INT)?)    -> { Token::Float(mk_f64(c)) }
        let INT:{()}       = ('-'|'+')? NAT                          -> {}
        let NAT:{()}       = digit+                                  -> {}
        let SPACES:{()}    = space*                                  -> {}
    );

    token()
}
