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

use proc_macro::TokenStream;

use crate::meta::syntax::{
    ASTParsec,
    ASTParsec::{
        PBind, PChar, PChoice, PCode, PIdent, PMap, POptional, PRepeat, PSequence, PString,
    },
    ASTParsecRule,
};

trait Transpile {
    fn transpile(&self) -> TokenStream;
}

impl Transpile for Vec<ASTParsecRule> {
    fn transpile(&self) -> TokenStream {
        unimplemented!()
    }
}

impl Transpile for ASTParsecRule {
    fn transpile(&self) -> TokenStream {
        let Self { name, returns, body } = self;

        unimplemented!()
    }
}

impl Transpile for ASTParsec {
    fn transpile(&self) -> TokenStream {
        match self {
            PBind(_, _) => unimplemented!(),
            PIdent(_) => unimplemented!(),
            PChar(_) => unimplemented!(),
            PString(_) => unimplemented!(),
            PCode(_) => unimplemented!(),
            PMap(_, _) => unimplemented!(),
            PSequence(_, _) => unimplemented!(),
            PChoice(_, _) => unimplemented!(),
            POptional(_) => unimplemented!(),
            PRepeat(_, _) => unimplemented!(),
        }
    }
}
