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

use celma_v0_core::parser::core::{eos, returns};
use celma_v0_core::parser::literal::{delimited_char, delimited_string};

use celma_v0_macro::parsec_rules;
use celma_v1_ast::syntax::ASTParsec::{
    PAtom, PAtoms, PBind, PCheck, PChoice, PCode, PEpsilon, PIdent, PMap, PNot, PRepeat, PSequence,
    PTry,
};
use celma_v1_ast::syntax::{ASTParsec, ASTParsecRule};

use celma_v0_core::parser::char::{alpha, digit};
use celma_v1_ast::syntax::ASTType::{PChar, POther, PUnit};

fn mk_rule(
    public: bool,
    name: String,
    input: Option<String>,
    returns: Option<String>,
    body: ASTParsec<char>,
) -> ASTParsecRule<char> {
    ASTParsecRule {
        public,
        name,
        input: input.map_or_else(|| PChar, POther),
        returns: returns.map_or_else(|| PUnit, POther),
        rule: body,
    }
}

fn mk_ast_parsec(
    bind: Option<String>,
    atom: ASTParsec<char>,
    occ: Option<char>,
    add: Option<(bool, ASTParsec<char>)>,
    trans: Option<String>,
) -> ASTParsec<char> {
    let occ = match occ {
        Some('?') => PChoice(atom.wrap(), PEpsilon().wrap()),
        Some('*') => PChoice(PRepeat(atom.wrap()).wrap(), PEpsilon().wrap()),
        Some('+') => PRepeat(atom.wrap()),
        _ => atom,
    };

    let bind = if let Some(value) = bind {
        PBind(value, occ.wrap())
    } else {
        occ
    };

    let add = if let Some((choice, parser)) = add {
        if choice {
            PChoice(bind.wrap(), parser.wrap())
        } else {
            PSequence(bind.wrap(), parser.wrap())
        }
    } else {
        bind
    };

    if let Some(value) = trans {
        PMap(add.wrap(), value)
    } else {
        add
    }
}

fn mk_atom(operation: Option<char>, parsec: ASTParsec<char>) -> ASTParsec<char> {
    match operation {
        Some('^') => PNot(parsec.wrap()),
        Some('!') => PTry(parsec.wrap()),
        Some('#') => PCheck(parsec.wrap()),
        _ => parsec,
    }
}

parsec_rules!(
    let skip = (' '|'\t'|'\n'|'\r')* -> {}
    let ident:{String} = (skip i=#(alpha (alpha|digit|'_')*) skip) -> { i.into_iter().collect() }

    let kind_content = (^('<'|'>')+ kind_content -> {})
              | ('<' kind_content '>' kind_content -> {})
              | ()

    let code_content = (^('}'|'{')+ code_content -> {})
              | ('{' code_content '}' code_content -> {})
              | ()

    let kind:{String} = (skip '<' c=#kind_content '>' skip) -> { c.into_iter().collect() }
    let code:{String} = (skip '{' c=#code_content '}' skip) -> { c.into_iter().collect() }

    let rules:{Vec<ASTParsecRule<char>>} = rule*
    let rule:{ASTParsecRule<char>} = (
        skip p="pub"? skip "let" skip n=ident i=kind? r=(':' _=kind)? '=' b=parsec skip
    ) -> { mk_rule(p.is_some(), n, i, r, b) }

    let parsec:{ASTParsec<char>} = (
        skip b=!(binding)? a=atom o=('?'|'*'|'+')? d=additional? t=transform? skip
    ) -> { mk_ast_parsec(b, a, o, d, t) }

    let binding:{String} = skip _=ident '=' skip
    let additional:{(bool,ASTParsec<char>)} = (skip c='|'? skip p=parsec) -> { (c.is_some(), p) }

    let atom:{ASTParsec<char>} = (
        skip o=('^'|'!'|'#')? skip p=(atom_block|atom_ident|atom_char|atom_string|atom_code) skip
    ) -> { mk_atom(o, p) }

    let atom_block:{ASTParsec<char>} = ('(' p=parsec? ')') -> { p.unwrap_or_else(PEpsilon) }
    let atom_ident:{ASTParsec<char>} = c=ident -> { PIdent(c) }
    let atom_char:{ASTParsec<char>} = c=delimited_char -> { PAtom(c) }
    let atom_string:{ASTParsec<char>} = c=delimited_string -> { PAtoms(c.chars().collect()) }
    let atom_code:{ASTParsec<char>} = c=code -> { PCode(c) }

    let transform:{String} = (skip "->" skip _=code)

    // Main entries
    let celma_parsec:{ASTParsec<char>} = (_=parsec eos)
    let celma_parsec_rules:{Vec<ASTParsecRule<char>>} = (_=rules eos)
);
