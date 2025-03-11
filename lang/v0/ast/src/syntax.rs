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
pub enum ASTParsec {
    PIdent(String),
    PAtom(char),
    PAtoms(Vec<char>),
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

impl ASTParsec {
    pub fn wrap(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ASTParsecRule {
    pub name: String,
    pub input: String,
    pub returns: String,
    pub rule: ASTParsec,
}
