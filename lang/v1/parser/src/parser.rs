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

use celma_core::parser::core::eos;
use celma_core::parser::literal::{delimited_char, delimited_string};

use crate::syntax::ASTParsec::{
    PBind, PChar, PCheck, PChoice, PCode, PIdent, PLookahead, PMap, PNot, POptional, PRepeat,
    PSequence, PString, PTry,
};
use crate::syntax::{ASTParsec, ASTParsecRule};
use celma_macro_v0::parsec_rules;

use celma_core::parser::char::space;
use celma_core::parser::char::{alpha, digit};

fn mk_rule(
    public: bool,
    name: String,
    input: Option<String>,
    returns: String,
    body: ASTParsec,
) -> ASTParsecRule {
    ASTParsecRule {
        public,
        name,
        input: input.unwrap_or(String::from("char")),
        returns,
        body: Box::new(body),
    }
}

fn mk_ast_parsec(
    bind: Option<String>,
    atom: ASTParsec,
    occ: Option<char>,
    add: Option<(bool, ASTParsec)>,
    trans: Option<String>,
) -> ASTParsec {
    let occ = match occ {
        Some('?') => POptional(Box::new(atom)),
        Some('*') => PRepeat(true, Box::new(atom)),
        Some('+') => PRepeat(false, Box::new(atom)),
        _ => atom,
    };

    let bind = if let Some(value) = bind {
        PBind(value, Box::new(occ))
    } else {
        occ
    };

    let add = if let Some(value) = add {
        if value.0 {
            PChoice(Box::new(bind), Box::new(value.1))
        } else {
            PSequence(Box::new(bind), Box::new(value.1))
        }
    } else {
        bind
    };

    if let Some(value) = trans {
        PMap(Box::new(add), value)
    } else {
        add
    }
}

fn mk_atom(operation: Option<char>, parsec: ASTParsec) -> ASTParsec {
    match operation {
        Some('^') => PNot(Box::new(parsec)),
        Some('!') => PTry(Box::new(parsec)),
        Some('#') => PCheck(Box::new(parsec)),
        Some('/') => PLookahead(Box::new(parsec)),
        _ => parsec,
    }
}

parsec_rules!(
    let skip:{()} = space* -> {}
    let ident:{String} = (skip i=#(alpha (alpha|digit|'_')*) skip) -> { i.into_iter().collect() }

    let kind:{String} = ('{' v=^'}'* '}') -> { v.into_iter().collect() }
    let code:{String} = ('{' c=^'}'* '}') -> { c.into_iter().collect() }

    let rules:{Vec<ASTParsecRule>} = rule*
    let rule:{ASTParsecRule} = (
        skip p="pub"? skip "let" n=ident i=kind? ':' r=kind '=' b=parsec skip
    ) -> { mk_rule(p.is_some(), n, i, r, b) }

    let parsec:{ASTParsec} = (
        skip b=!(binding)? a=atom o=('?'|'*'|'+')? d=additional? t=transform? skip
    ) -> { mk_ast_parsec(b, a, o, d, t) }

    let binding:{String} = skip _=ident '=' skip
    let additional:{(bool,ASTParsec)} = (skip c='|'? skip p=parsec) -> { (c.is_some(), p) }

    let atom:{ASTParsec} = (
        skip o=('^'|'!'|'#'|'/')? skip p=(atom_block|atom_char|atom_string|atom_code) skip
    ) -> { mk_atom(o, p) }

    let atom_block:{ASTParsec} = '(' _=parsec ')'
    let atom_char:{ASTParsec} = c=delimited_char -> { PChar(c) }
    let atom_string:{ASTParsec} = c=delimited_string -> { PString(c) }
    let atom_code:{ASTParsec} = c=code -> { PCode(c) }
    let atom_ident:{ASTParsec} = c=ident -> { PIdent(c) }

    let transform:{String} = (skip "->" skip _=code)

    // Main entries
    let celma_parsec:{ASTParsec} = (_=parsec eos)
    let celma_parsec_rules:{Vec<ASTParsecRule>} = (_=rules eos)
);
