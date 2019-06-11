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

use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::meta::syntax::ASTParsec;
use crate::meta::syntax::ASTParsec::{
    PBind, PChar, PChoice, PCode, PIdent, PMap, POptional, PRepeat, PSequence, PString,
};
use crate::meta::syntax::ASTParsecRule;

pub trait Transpile<E> {
    fn transpile(&self) -> E;
}

impl Transpile<TokenStream> for Vec<ASTParsecRule> {
    fn transpile(&self) -> TokenStream {
        self.iter().map(|a| a.transpile()).collect()
    }
}

impl Transpile<TokenStream> for ASTParsecRule {
    fn transpile(&self) -> TokenStream {
        let Self {
            name,
            returns,
            body,
        } = self;

        let name = syn::Ident::new(name.as_str(), Span::call_site());
        let returns = syn::parse_str::<TokenStream>(returns.as_str()).unwrap();
        let (_, body) = body.transpile();

        quote!(
            fn #name<'a,S:'a>() -> imp Parse<#returns,S> + Combine<#returns> + 'a
                where S:Stream<Item=char>
            {
                #body
            }
        )
    }
}

impl Transpile<(Option<String>, TokenStream)> for ASTParsec {
    fn transpile(&self) -> (Option<String>, TokenStream) {
        match self {
            PBind(n, p) => (Some(n.clone()), p.transpile().1),
            PIdent(n) => {
                let n = syn::Ident::new(n, Span::call_site());
                (None, quote!(lazy(|| #n())))
            }
            PChar(c) => (None, quote!(char(#c))),
            PString(s) => (None, quote!(string(#s))),
            PCode(c) => {
                let c = syn::parse_str::<TokenStream>(c.as_str()).unwrap();
                (None, quote!(#c))
            }
            PMap(p, c) => {
                let (pp, pt) = p.transpile();
                (None, quote!(#pt.fmap({{ | #pp | #c }})))
            }
            PSequence(l, r) => {
                let (lp, lt) = l.transpile();
                let (rp, rt) = r.transpile();

                if lp.is_none() {
                    (rp, quote!(#lt.and_right(#rt)))
                } else if rp.is_none() {
                    (lp, quote!(#lt.and_left(#rt)))
                } else {
                    (
                        Some(format!("({},{})", lp.unwrap(), rp.unwrap())),
                        quote!(#lt.and(#rt)),
                    )
                }
            }
            PChoice(l, r) => {
                let (_, lt) = l.transpile();
                let (_, rt) = r.transpile();

                (None, quote!(#lt.or(#rt)))
            }
            POptional(p) => {
                let (_, pt) = p.transpile();
                (None, quote!(#pt.opt()))
            }
            PRepeat(b, p) => {
                let (_, pt) = p.transpile();
                if *b {
                    (None, quote!(#pt.opt_rep()))
                } else {
                    (None, quote!(#pt.rep()))
                }
            }
        }
    }
}
