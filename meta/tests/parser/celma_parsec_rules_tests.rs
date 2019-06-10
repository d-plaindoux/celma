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
    use celma_core::parser::response::Response::{Reject, Success};
    use celma_core::stream::char_stream::CharStream;
    use celma_lang::meta::parser::celma_parsec_rules;
    use celma_lang::meta::syntax::ASTParsec::{PChar, PChoice, PCode};
    use celma_lang::meta::syntax::ASTParsecRule;

    #[test]
    fn it_parse_one_char_rule() {
        let response = celma_parsec_rules().parse(CharStream::new("let a:{char} ::= {char('a')}"));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                vec!(ASTParsecRule {
                    name: String::from("a"),
                    returns: String::from("char"),
                    body: Box::new(PCode(String::from("char(\'a\')"))),
                })
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_two_char_rules() {
        let response = celma_parsec_rules().parse(CharStream::new(
            "let a:{char} ::= {char('a')} let b:{char} ::= {char('b')}",
        ));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                vec!(
                    ASTParsecRule {
                        name: String::from("a"),
                        returns: String::from("char"),
                        body: Box::new(PCode(String::from("char(\'a\')"))),
                    },
                    ASTParsecRule {
                        name: String::from("b"),
                        returns: String::from("char"),
                        body: Box::new(PCode(String::from("char(\'b\')"))),
                    }
                )
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_two_complexe_rules() {
        let response = celma_parsec_rules().parse(CharStream::new(
            "let a:{char} ::= 'a'|{char('b')} let b:{char} ::= {char('c')}",
        ));

        match response {
            Success(ast, _, _) => assert_eq!(
                ast,
                vec!(
                    ASTParsecRule {
                        name: String::from("a"),
                        returns: String::from("char"),
                        body: Box::new(PChoice(
                            Box::new(PChar('a')),
                            Box::new(PCode(String::from("char(\'b\')"))),
                        )),
                    },
                    ASTParsecRule {
                        name: String::from("b"),
                        returns: String::from("char"),
                        body: Box::new(PCode(String::from("char(\'c\')"))),
                    }
                )
            ),
            _ => assert_eq!(true, false),
        };
    }

    #[test]
    fn it_parse_celma_rules() {
        let response = celma_parsec_rules().parse(CharStream::new(
            r#"
        let parsec_rules:{Vec<ASTParserRule>} ::= _=parsec_rule+
        let parsec_rule:{ASTParserRule}       ::= "let" n=ident ':' '{' t=rust_code '}' "::=" p=parsec => { ASTParserRule(n,c,p) }
        let parsec:{ASTParser}                ::= binding? atom occurrence? additional? transform?
        let binding:{String}                  ::= _=ident '='
        let occurrence:{char}                 ::= ('*' | '+' | '?')
        let additional:{(bool,ASTParser)}     ::= (c=("|"?) => { c.is_empty() }) _=parser
        let transform:{String}                ::= "=>" '{' _=rust_code '}'
        let atom:{ASTParser}                  ::= ('(' _=parser ')') | _=CHAR | _=STRING | _=ident | ('{' _=rust_code '}')
            "#
        ));

        match response {
            Success(_, _, _) => assert_eq!(true, true),
            Reject(_, _) => assert_eq!(true, false),
        };
    }
}
