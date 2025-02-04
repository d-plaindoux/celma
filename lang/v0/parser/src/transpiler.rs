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

extern crate proc_macro;

use celma_lang_ast::syntax::ASTParsec::{
    PAtom, PAtoms, PBind, PCheck, PChoice, PCode, PIdent, PLookahead, PMap, PNot, POptional,
    PRepeat, PSequence, PTry,
};
use celma_lang_ast::syntax::{ASTParsec, ASTParsecRule};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Error;

pub trait Transpile<E> {
    fn transpile(&self) -> Result<E, Error>;
}

impl Transpile<TokenStream> for Vec<ASTParsecRule<char>> {
    fn transpile(&self) -> Result<TokenStream, Error> {
        let parsers: TokenStream = self
            .iter()
            .map(|a| a.transpile())
            .collect::<Result<TokenStream, Error>>()?;

        Ok(quote!(
            #parsers
        ))
    }
}

impl Transpile<TokenStream> for ASTParsecRule<char> {
    fn transpile(&self) -> Result<TokenStream, Error> {
        let Self {
            name,
            input,
            returns,
            rule: body,
            ..
        } = self;

        let name = syn::Ident::new(name.as_str(), Span::call_site());
        let input = syn::parse_str::<TokenStream>(input.as_str())?;
        let returns = syn::parse_str::<TokenStream>(returns.as_str())?;
        let body = body.transpile_body()?.1;

        Ok(quote!(
            pub fn #name<'a,S:'a>() -> impl celma_core::parser::specs::Parse<#returns,S> +
                                            celma_core::parser::specs::Combine<#returns> +
                                            'a
                where S:celma_core::stream::specs::Stream<Item=#input>,
            {
                use celma_core::parser::a_try::a_try;
                use celma_core::parser::and::AndOperation;
                use celma_core::parser::check::check;
                use celma_core::parser::lookahead::lookahead;
                use celma_core::parser::map::MapOperation;
                use celma_core::parser::not::NotOperation;
                use celma_core::parser::option::OptionalOperation;
                use celma_core::parser::or::OrOperation;
                use celma_core::parser::repeat::RepeatOperation;
                use celma_core::parser::specs::Parse;

                celma_core::parser::core::parser(#body)
            }
        ))
    }
}

impl Transpile<TokenStream> for ASTParsec<char> {
    fn transpile(&self) -> Result<TokenStream, Error> {
        let body = self.transpile_body()?.1;

        Ok(quote!(
            {
                use celma_core::parser::a_try::a_try;
                use celma_core::parser::and::AndOperation;
                use celma_core::parser::check::check;
                use celma_core::parser::map::MapOperation;
                use celma_core::parser::not::NotOperation;
                use celma_core::parser::option::OptionalOperation;
                use celma_core::parser::or::OrOperation;
                use celma_core::parser::repeat::RepeatOperation;
                use celma_core::parser::specs::Parse;

                celma_core::parser::core::parser(#body)
            }
        ))
    }
}

pub trait TranspileBody<E> {
    fn transpile_body(&self) -> Result<E, Error>;
}

impl TranspileBody<(Option<String>, TokenStream)> for ASTParsec<char> {
    fn transpile_body(&self) -> Result<(Option<String>, TokenStream), Error> {
        match self {
            PBind(n, p) => Ok((Some(n.clone()), p.transpile_body()?.1)),
            PIdent(n) => {
                let n = syn::Ident::new(n, Span::call_site());
                Ok((None, quote!(celma_core::parser::lazy::lazy(|| #n()))))
            }
            PAtom(c) => Ok((None, quote!(celma_core::parser::char::a_char(#c)))),
            PAtoms(s) => {
                let s = s.into_iter().collect::<String>();
                Ok((None, quote!(celma_core::parser::literal::string(#s))))
            }
            PCode(c) => {
                let c = syn::parse_str::<TokenStream>(c.as_str()).unwrap();
                Ok((None, quote!(#c)))
            }
            PMap(p, c) => {
                let (pp, pt) = p.transpile_body()?;
                let c = syn::parse_str::<TokenStream>(c.as_str())?;

                if let Some(p) = pp {
                    let pp = syn::parse_str::<TokenStream>(p.as_str())?;
                    Ok((None, quote!(#pt.map(|#pp|{ #c }))))
                } else {
                    Ok((None, quote!(#pt.map(|_|{ #c }))))
                }
            }
            PSequence(l, r) => {
                let (lp, lt) = l.transpile_body()?;
                let (rp, rt) = r.transpile_body()?;

                if lp.is_none() {
                    Ok((rp, quote!(#lt.and_right(#rt))))
                } else if rp.is_none() {
                    Ok((lp, quote!(#lt.and_left(#rt))))
                } else {
                    Ok((
                        Some(format!("({},{})", lp.unwrap(), rp.unwrap())),
                        quote!(#lt.and(#rt)),
                    ))
                }
            }
            PChoice(l, r) => {
                let (_, lt) = l.transpile_body()?;
                let (_, rt) = r.transpile_body()?;
                Ok((None, quote!(#lt.or(#rt))))
            }
            PNot(p) => {
                let (_, pt) = p.transpile_body()?;
                Ok((None, quote!(#pt.not())))
            }
            PTry(p) => {
                let (_, pt) = p.transpile_body()?;
                Ok((None, quote!(a_try(#pt))))
            }
            PCheck(p) => {
                let (_, pt) = p.transpile_body()?;
                Ok((None, quote!(check(#pt))))
            }
            POptional(p) => {
                let (_, pt) = p.transpile_body()?;
                Ok((None, quote!(#pt.opt())))
            }
            PRepeat(b, p) => {
                let (_, pt) = p.transpile_body()?;
                if *b {
                    Ok((None, quote!(#pt.opt_rep())))
                } else {
                    Ok((None, quote!(#pt.rep())))
                }
            }
            PLookahead(p) => {
                let (_, pt) = p.transpile_body()?;
                Ok((None, quote!(lookahead(#pt))))
            }
        }
    }
}
