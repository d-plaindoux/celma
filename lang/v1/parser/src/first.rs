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

use crate::token::Token::{AllAtom, Atom};
use crate::token::{First, Token};
use celma_v1_ast::syntax::ASTParsec;

impl<E> First<E> for ASTParsec<E>
where
    E: Copy,
{
    fn first(&self) -> Vec<Token<E>> {
        match self {
            ASTParsec::PIdent(_) => vec![AllAtom],
            ASTParsec::PAtom(c) => vec![Atom(*c)],
            ASTParsec::PAtoms(s) => vec![Atom(s[0])], // /!\ Should not be empty !
            ASTParsec::PBind(_, p) => p.first(),
            ASTParsec::PCode(_) => vec![AllAtom],
            ASTParsec::PMap(p, _) => p.first(),
            ASTParsec::PSequence(p, q) => {
                let p = p.first();
                if !p.is_empty() {
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
