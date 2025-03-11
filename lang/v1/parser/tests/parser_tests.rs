/*
 * Copyright 2019-2025 Didier Plaindoux
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
mod parser_tests {
    use celma_v0_core::parser::specs::Parse;
    use celma_v0_core::stream::char_stream::CharStream;
    use celma_v0_core::stream::specs::Len;
    use celma_v1::parser::{
        atom_char, atom_code, atom_ident, atom_string, code, code_content, kind, parsec, rule,
    };
    use celma_v1_ast::syntax::ASTParsec::{
        PAtom, PAtoms, PBind, PCheck, PChoice, PCode, PEpsilon, PIdent, PNot, PRepeat, PSequence,
        PTry,
    };
    use celma_v1_ast::syntax::ASTParsecRule;
    use celma_v1_ast::syntax::ASTType::{PChar, PUnit};

    #[test]
    fn should_parse_kind() {
        let response = kind().parse(CharStream::new("<hello>"));

        assert_eq!(response.fold(|v, _, _| v == "hello", |_, _| false), true);
    }

    #[test]
    fn should_parse_atom_ident() {
        let response = atom_ident().parse(CharStream::new("hello"));

        assert_eq!(
            response.fold(|v, _, _| v == PIdent(String::from("hello")), |_, _| false),
            true
        );
    }

    #[test]
    fn should_parse_atom_code() {
        let response = atom_code().parse(CharStream::new("{hello()}"));

        assert_eq!(
            response.fold(|v, _, _| v == PCode(String::from("hello()")), |_, _| false),
            true
        );
    }

    #[test]
    fn should_parse_atom_char() {
        let response = atom_char().parse(CharStream::new("'a'"));

        assert_eq!(response.fold(|v, _, _| v == PAtom('a'), |_, _| false), true);
    }

    #[test]
    fn should_parse_atom_string() {
        let response = atom_string().parse(CharStream::new("\"test\""));

        assert_eq!(
            response.fold(
                |v, _, _| v == PAtoms("test".chars().collect()),
                |_, _| false
            ),
            true
        );
    }

    #[test]
    fn should_parse_ident_empty_code_content() {
        let response = code_content().parse(CharStream::new("}"));

        assert_eq!(response.fold(|_, s, _| s.len() == 1, |_, _| false), true);
    }

    #[test]
    fn should_parse_ident_body_with_block_code_content() {
        let response = code_content().parse(CharStream::new("tutu { titi } toto }"));

        assert_eq!(response.fold(|_, s, _| s.len() == 1, |_, _| false), true);
    }

    #[test]
    fn should_parse_ident_body_with_block_unbalanced_code_content() {
        let response = code_content().parse(CharStream::new("{ titi"));

        assert_eq!(response.fold(|_, _, _| false, |_, _| true), true);
    }

    #[test]
    fn should_parse_ident_body_code() {
        let response = code().parse(CharStream::new("{ titi }"));

        assert_eq!(
            response.fold(|v, _, _| v == String::from(" titi "), |_, _| false),
            true
        );
    }

    #[test]
    fn should_parse_ident_body_code_with_block() {
        let response = code().parse(CharStream::new("{ {titi} }"));

        assert_eq!(
            response.fold(
                |v, s, _| v == String::from(" {titi} ") && s.is_empty(),
                |_, _| false
            ),
            true
        );
    }

    #[test]
    fn should_parse_ident_body() {
        let response = parsec().parse(CharStream::new("entry"));

        assert_eq!(
            response.fold(|v, _, _| v == PIdent(String::from("entry")), |_, _| false),
            true
        );
    }

    #[test]
    fn should_parse_sequence_body() {
        let response = parsec().parse(CharStream::new("entry1 entry2"));

        assert_eq!(
            response.fold(
                |v, _, _| v
                    == PSequence(
                        PIdent(String::from("entry1")).wrap(),
                        PIdent(String::from("entry2")).wrap(),
                    ),
                |_, _| false
            ),
            true
        );
    }

    #[test]
    fn should_parse_choice_body() {
        let response = parsec().parse(CharStream::new("entry1 | entry2"));

        assert_eq!(
            response.fold(
                |v, _, _| v
                    == PChoice(
                        PIdent(String::from("entry1")).wrap(),
                        PIdent(String::from("entry2")).wrap(),
                    ),
                |_, _| false
            ),
            true
        );
    }

    #[test]
    fn should_parse_optional_ident_body() {
        let response = parsec().parse(CharStream::new("entry?"));

        assert_eq!(
            response.fold(
                |v, _, _| v == PChoice(PIdent(String::from("entry")).wrap(), PEpsilon().wrap()),
                |_, _| false
            ),
            true
        );
    }

    #[test]
    fn should_parse_repeatable_ident_body() {
        let response = parsec().parse(CharStream::new("entry+"));

        assert_eq!(
            response.fold(
                |v, _, _| v == PRepeat(PIdent(String::from("entry")).wrap()),
                |_, _| false
            ),
            true
        );
    }

    #[test]
    fn should_parse_optional_repeatable_ident_body() {
        let response = parsec().parse(CharStream::new("entry*"));

        assert_eq!(
            response.fold(
                |v, _, _| v
                    == PChoice(
                        PRepeat(PIdent(String::from("entry")).wrap()).wrap(),
                        PEpsilon().wrap()
                    ),
                |_, _| false
            ),
            true
        );
    }
    #[test]
    fn should_parse_bind_optional_ident_body() {
        let response = parsec().parse(CharStream::new("a=entry?"));

        assert_eq!(
            response.fold(
                |v, _, _| v
                    == PBind(
                        String::from("a"),
                        PChoice(PIdent(String::from("entry")).wrap(), PEpsilon().wrap()).wrap()
                    ),
                |_, _| false
            ),
            true
        );
    }

    #[test]
    fn should_parse_bind_optional_ident_body_with_capture_all_chars() {
        let response = parsec().parse(CharStream::new("a=#entry"));

        assert_eq!(
            response.fold(
                |v, _, _| v
                    == PBind(
                        String::from("a"),
                        PCheck(PIdent(String::from("entry")).wrap()).wrap()
                    ),
                |_, _| false
            ),
            true
        );
    }

    #[test]
    fn should_parse_bind_optional_ident_body_with_negation() {
        let response = parsec().parse(CharStream::new("a=^entry"));

        assert_eq!(
            response.fold(
                |v, _, _| v
                    == PBind(
                        String::from("a"),
                        PNot(PIdent(String::from("entry")).wrap()).wrap()
                    ),
                |_, _| false
            ),
            true
        );
    }

    #[test]
    fn should_parse_bind_optional_ident_body_with_try() {
        let response = parsec().parse(CharStream::new("a=!entry"));

        assert_eq!(
            response.fold(
                |v, _, _| v
                    == PBind(
                        String::from("a"),
                        PTry(PIdent(String::from("entry")).wrap()).wrap()
                    ),
                |_, _| false
            ),
            true
        );
    }

    #[test]
    fn should_parse_epsilon() {
        let response = parsec().parse(CharStream::new("()"));

        assert_eq!(response.fold(|v, _, _| v == PEpsilon(), |_, _| false), true);
    }

    #[test]
    fn should_parse_protected_simple_rule() {
        let response = rule().parse(CharStream::new("let x = entry"));

        assert_eq!(
            response.fold(
                |v, _, _| v
                    == ASTParsecRule::<char> {
                        public: false,
                        name: String::from("x"),
                        input: PChar,
                        returns: PUnit,
                        rule: PIdent(String::from("entry"))
                    },
                |_, _| false
            ),
            true
        );
    }

    #[test]
    fn should_parse_public_simple_rule() {
        let response = rule().parse(CharStream::new("pub let x = entry"));

        assert_eq!(
            response.fold(
                |v, _, _| v
                    == ASTParsecRule::<char> {
                        public: true,
                        name: String::from("x"),
                        input: PChar,
                        returns: PUnit,
                        rule: PIdent(String::from("entry"))
                    },
                |_, _| false
            ),
            true
        );
    }
}
