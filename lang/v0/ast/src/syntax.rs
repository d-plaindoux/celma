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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ASTParsec<I> {
    PIdent(String),
    // Are these constructors required?
    PAtom(I),
    PAtoms(Vec<I>),
    PBind(String, Box<ASTParsec<I>>),
    // --------------------------------
    PCode(String),
    PMap(Box<ASTParsec<I>>, String),
    PSequence(Box<ASTParsec<I>>, Box<ASTParsec<I>>),
    PChoice(Box<ASTParsec<I>>, Box<ASTParsec<I>>),
    PNot(Box<ASTParsec<I>>),
    PTry(Box<ASTParsec<I>>),
    PCheck(Box<ASTParsec<I>>),
    POptional(Box<ASTParsec<I>>),
    PRepeat(bool, Box<ASTParsec<I>>),
    PLookahead(Box<ASTParsec<I>>),
}



impl<I> ASTParsec<I> {
    pub fn wrap(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ASTParsecRule<I> {
    pub name: String,
    pub input: String,
    pub returns: String,
    pub rule: ASTParsec<I>,
}
