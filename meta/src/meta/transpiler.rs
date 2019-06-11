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

extern crate proc_macro;

use quote::quote;

use crate::meta::syntax::ASTParsec;
use crate::meta::syntax::ASTParsec::{
    PBind, PChar, PChoice, PCode, PIdent, PMap, POptional, PRepeat, PSequence, PString,
};
use crate::meta::syntax::ASTParsecRule;

pub trait Transpile {
    fn transpile(&self) -> String;
}

impl Transpile for Vec<ASTParsecRule> {
    fn transpile(&self) -> String {
        self.iter().map(|a| a.transpile()).collect()
    }
}

impl ASTParsecRule {
    pub fn transpile(&self) -> String {
        let Self {
            name,
            returns,
            body,
        } = self;
        let (_, body_t) = body.transpile();

        quote!(
            fn #name<'a,S:'a>() -> imp Parse<#returns,S> + Combine<#returns> + 'a
                where S:Stream<Item=char>
            {
                #body_t
            }
        )
        .to_string()
    }
}

impl ASTParsec {
    pub fn transpile(&self) -> (String, String) {
        match self {
            PBind(n, p) => (n.clone(), p.transpile().1),
            PIdent(n) => (String::from(""), quote!(lazy(||#n())).to_string()),
            PChar(c) => (String::from(""), format!("char('{}')", c)),
            PString(_s) => (String::from(""), quote!(string("#_s")).to_string()),
            PCode(c) => (String::from(""), c.clone()),
            PMap(p, c) => {
                let (pp, pt) = p.transpile();
                (
                    String::from(""),
                    quote!(#pt.fmap({{ |#pp| #c }})).to_string(),
                )
            }
            PSequence(l, r) => {
                let (lp, lt) = l.transpile();
                let (rp, rt) = r.transpile();

                if lp.clone().is_empty() {
                    (rp, format!("{}.and_right({})", lt, rt))
                } else if rp.clone().is_empty() {
                    (lp, format!("{}.and_left({})", lt, rt))
                } else {
                    (format!("({},{})", lp, rp), format!("{}.and({})", lt, rt))
                }
            }
            PChoice(l, r) => {
                let (_, lt) = l.transpile();
                let (_, rt) = r.transpile();

                (String::from(""), format!("{}.or({})", lt, rt))
            }
            POptional(p) => {
                let (_, pt) = p.transpile();
                (String::from(""), format!("{}.opt()", pt))
            }
            PRepeat(b, p) => {
                let (_, pt) = p.transpile();
                if *b {
                    (String::from(""), format!("{}.opt_rep()", pt))
                } else {
                    (String::from(""), format!("{}.rep()", pt))
                }
            }
        }
    }
}
