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

use celma_core::parser::ff::Token::Atom;
use celma_core::parser::ff::{First, Token};
use celma_core::stream::stream::Stream;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ASTParsec {
    PIdent(String),
    PChar(char),
    PString(String),
    PBind(String, Box<ASTParsec>),
    PCode(String),
    PMap(Box<ASTParsec>, String),
    PSequence(Box<ASTParsec>, Box<ASTParsec>),
    PChoice(Box<ASTParsec>, Box<ASTParsec>),
    PNot(Box<ASTParsec>),
    PTry(Box<ASTParsec>),
    PCheck(Box<ASTParsec>),
    POptional(Box<ASTParsec>),
    PRepeat(bool, Box<ASTParsec>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ASTParsecRule {
    pub name: String,
    pub input: String,
    pub returns: String,
    pub body: Box<ASTParsec>,
}

/*
impl<S> First<S> for ASTParsec
where
    S: Stream<Item = char>,
{
    fn first(&self) -> Vec<Token<char>> {
        match self {
            ASTParsec::PIdent(_) => vec![],
            ASTParsec::PChar(c) => vec![Atom(c.clone())],
            ASTParsec::PString(s) => s.chars().next().map(|c| vec![Atom(c)]).unwrap_or(vec![]),
            ASTParsec::PBind(_, p) => p.first(),
            ASTParsec::PCode(_) => vec![],
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
            ASTParsec::PTry(_) => vec![],
            ASTParsec::PCheck(_) => vec![],
            ASTParsec::POptional(_) => vec![],
            ASTParsec::PRepeat(_, _) => vec![],
        }
    }
}
*/
