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

#![feature(proc_macro_hygiene)]

use celma_plugin::parsec_rules;

fn main() {
    parsec_rules!(
        let skip                              ::= (' '|'\r'|'\n'|'\t')*
        let parsec_rules:{Vec<ASTParserRule>} ::= _=parsec_rule+
        let parsec_rule:{ASTParserRule}       ::= "let" n=ident ':' '{' t=rust_code '}' "::=" p=parsec => { ASTParserRule(n,c,p) }
        let parsec:{ASTParser}                ::= binding? atom occurrence? additional? transform?
        let binding:{String}                  ::= _=ident '='
        let occurrence:{char}                 ::= ('*' | '+' | '?')
        let additional:{(bool,ASTParser)}     ::= _=(c=("|"?) => { c.is_empty() }) _=parser
        let transform:{String}                ::= "=>" '{' _=rust_code '}'
        let atom:{ASTParser}                  ::= ('(' _=parser ')') | _=CHAR | _=STRING | _=NUMBER | _=ident | ('{' _=rust_code '}')
        let ident:{String}                    ::= _=[a..z_A..Z]+ - {"let"}
    );
}
