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

use crate::meta::syntax::ASTParsec;
use crate::meta::token::Token::{AllAtom, Atom};
use crate::meta::token::{First, Token};

impl First<char> for ASTParsec {
    fn first(&self) -> Vec<Token<char>> {
        match self {
            ASTParsec::PIdent(_) => vec![AllAtom],
            ASTParsec::PChar(c) => vec![Atom(c.clone())],
            ASTParsec::PString(s) => s.chars().next().map(|c| vec![Atom(c)]).unwrap_or(vec![]),
            ASTParsec::PBind(_, p) => p.first(),
            ASTParsec::PCode(_) => vec![AllAtom],
            ASTParsec::PMap(p, _) => p.first(),
            ASTParsec::PSequence(p, q) => {
                let p = p.first();
                if p.len() > 0 {
                    return p;
                }
                q.first()
            }
            ASTParsec::PChoice(p, q) => {
                let mut p = p.first();
                p.append(&mut q.first());
                p
            }
            ASTParsec::PNot(_) => vec![],
            ASTParsec::PTry(p) => p.first(),
            ASTParsec::PCheck(p) => p.first(),
            ASTParsec::POptional(p) => p.first(),     // TODO
            ASTParsec::PRepeat(true, p) => p.first(), // TODO
            ASTParsec::PRepeat(false, p) => p.first(),
            ASTParsec::PLookahead(p) => p.first(),
        }
    }
}
