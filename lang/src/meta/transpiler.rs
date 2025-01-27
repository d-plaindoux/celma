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

extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::meta::syntax::ASTParsec;
use crate::meta::syntax::ASTParsec::{
    PBind, PChar, PCheck, PChoice, PCode, PIdent, PLookahead, PMap, PNot, POptional, PRepeat,
    PSequence, PString, PTry,
};
use crate::meta::syntax::ASTParsecRule;

pub trait Transpile<E> {
    fn transpile(&self) -> E;
}

impl Transpile<TokenStream> for Vec<ASTParsecRule> {
    fn transpile(&self) -> TokenStream {
        let parsers: TokenStream = self.iter().map(|a| a.transpile()).collect();

        quote!(
            #parsers
        )
    }
}

impl Transpile<TokenStream> for ASTParsecRule {
    fn transpile(&self) -> TokenStream {
        let Self {
            name,
            input,
            returns,
            body,
        } = self;

        let name = syn::Ident::new(name.as_str(), Span::call_site());
        let input = syn::parse_str::<TokenStream>(input.as_str()).unwrap();
        let returns = syn::parse_str::<TokenStream>(returns.as_str()).unwrap();
        let body = body.transpile_body().1;

        quote!(
            pub fn #name<'a,S:'a>() -> impl celma_core::parser::specs::Parse<#returns,S> +
                                            celma_core::parser::specs::Combine<#returns> +
                                            'a
                where S:celma_core::stream::specs::Stream<Item=#input>,
            {
                use celma_core::parser::a_try::a_try;
                use celma_core::parser::and::AndOperation;
                use celma_core::parser::check::check;
                use celma_core::parser::lookahead::lookahead;
                use celma_core::parser::fmap::FMapOperation;
                use celma_core::parser::not::NotOperation;
                use celma_core::parser::option::OptionalOperation;
                use celma_core::parser::or::OrOperation;
                use celma_core::parser::repeat::RepeatOperation;
                use celma_core::parser::specs::Parse;

                celma_core::parser::core::parser(#body)
            }
        )
    }
}

impl Transpile<TokenStream> for ASTParsec {
    fn transpile(&self) -> TokenStream {
        let body = self.transpile_body().1;

        quote!(
            {
                use celma_core::parser::a_try::a_try;
                use celma_core::parser::and::AndOperation;
                use celma_core::parser::check::check;
                use celma_core::parser::fmap::FMapOperation;
                use celma_core::parser::not::NotOperation;
                use celma_core::parser::option::OptionalOperation;
                use celma_core::parser::or::OrOperation;
                use celma_core::parser::repeat::RepeatOperation;
                use celma_core::parser::specs::Parse;

                celma_core::parser::core::parser(#body)
            }
        )
    }
}

pub trait TranspileBody<E> {
    fn transpile_body(&self) -> E;
}

impl TranspileBody<(Option<String>, TokenStream)> for ASTParsec {
    fn transpile_body(&self) -> (Option<String>, TokenStream) {
        match self {
            PBind(n, p) => (Some(n.clone()), p.transpile_body().1),
            PIdent(n) => {
                let n = syn::Ident::new(n, Span::call_site());
                (None, quote!(celma_core::parser::lazy::lazy(|| #n())))
            }
            PChar(c) => (None, quote!(celma_core::parser::char::a_char(#c))),
            PString(s) => (None, quote!(celma_core::parser::literal::string(#s))),
            PCode(c) => {
                let c = syn::parse_str::<TokenStream>(c.as_str()).unwrap();
                (None, quote!(#c))
            }
            PMap(p, c) => {
                let (pp, pt) = p.transpile_body();
                let c = syn::parse_str::<TokenStream>(c.as_str()).unwrap();

                if pp.is_none() {
                    (None, quote!(#pt.fmap(|_|{ #c })))
                } else {
                    let pp = syn::parse_str::<TokenStream>(pp.unwrap().as_str()).unwrap();
                    (None, quote!(#pt.fmap(|#pp|{ #c })))
                }
            }
            PSequence(l, r) => {
                let (lp, lt) = l.transpile_body();
                let (rp, rt) = r.transpile_body();

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
                let (_, lt) = l.transpile_body();
                let (_, rt) = r.transpile_body();

                (None, quote!(#lt.or(#rt)))
            }
            PNot(p) => {
                let (_, pt) = p.transpile_body();
                (None, quote!(#pt.not()))
            }
            PTry(p) => {
                let (_, pt) = p.transpile_body();
                (None, quote!(a_try(#pt)))
            }
            PCheck(p) => {
                let (_, pt) = p.transpile_body();
                (None, quote!(check(#pt)))
            }
            POptional(p) => {
                let (_, pt) = p.transpile_body();
                (None, quote!(#pt.opt()))
            }
            PRepeat(b, p) => {
                let (_, pt) = p.transpile_body();
                if *b {
                    (None, quote!(#pt.opt_rep()))
                } else {
                    (None, quote!(#pt.rep()))
                }
            }
            PLookahead(p) => {
                let (_, pt) = p.transpile_body();
                (None, quote!(lookahead(#pt)))
            }
        }
    }
}
