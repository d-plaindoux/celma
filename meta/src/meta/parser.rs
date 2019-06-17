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

use celma_core::parser::a_try::a_try;
use celma_core::parser::and::{AndOperation, AndProjection};
use celma_core::parser::bind::BindOperation;
use celma_core::parser::char::{char, char_in_range, char_in_set, not_char};
use celma_core::parser::core::{eos, fail, parser, returns};
use celma_core::parser::fmap::FMapOperation;
use celma_core::parser::lazy::lazy;
use celma_core::parser::literal::{delimited_string, string};
use celma_core::parser::option::OptionalOperation;
use celma_core::parser::or::OrOperation;
use celma_core::parser::parser::{Combine, Parse};
use celma_core::parser::repeat::RepeatOperation;
use celma_core::stream::stream::Stream;

use crate::meta::syntax::ASTParsec::{
    PBind, PChar, PCheck, PChoice, PCode, PIdent, PMap, PNot, POptional, PRepeat, PSequence,
    PString, PTry,
};
use crate::meta::syntax::{ASTParsec, ASTParsecRule};

#[inline]
fn skip<'a, S: 'a>() -> impl Parse<(), S> + Combine<()> + Clone + 'a
where
    S: Stream<Item = char>,
{
    char_in_set(vec!['\n', '\r', '\t', ' '])
        .opt_rep()
        .fmap({ |_| () })
}

#[inline]
fn ident<'a, S: 'a>() -> impl Parse<String, S> + Combine<String> + Clone + 'a
where
    S: Stream<Item = char>,
{
    (char_in_range('A'..'Z')
        .or(char_in_range('a'..'z'))
        .or(char_in_range('0'..'9'))
        .or(char('_')))
    .rep()
    .fmap(|v| v.into_iter().collect())
    .bind(|s| {
        if s == String::from("let") {
            parser(fail(false))
        } else {
            parser(returns(s))
        }
    })
}

#[inline]
fn delimited_char<'a, S: 'a>() -> impl Parse<char, S> + Combine<char> + Clone + 'a
where
    S: Stream<Item = char>,
{
    char('\'')
        .and_right(string("\'").fmap(|_| '\'').or(not_char('\'')))
        .and_left(char('\''))
}

// -------------------------------------------------------------------------------------------------

#[inline]
fn parsec_rules<'a, S: 'a>(
) -> impl Parse<Vec<ASTParsecRule>, S> + Combine<Vec<ASTParsecRule>> + Clone + 'a
where
    S: Stream<Item = char>,
{
    string("let")
        .and_left(skip())
        .and_right(ident())
        .and_left(skip())
        .and(code().opt())
        .and_left(skip())
        .and_left(char(':'))
        .and_left(skip())
        .and(code())
        .and_left(skip())
        .and_left(string("="))
        .and_left(skip())
        .and(parsec())
        .and_left(skip())
        .fmap(
            |(((n, i), r), b): (((String, Option<String>), String), ASTParsec)| ASTParsecRule {
                name: n,
                input: i.unwrap_or(String::from("char")),
                returns: r,
                body: Box::new(b),
            },
        )
        .rep()
}

// -------------------------------------------------------------------------------------------------

#[inline]
fn parsec<'a, S: 'a>() -> impl Parse<ASTParsec, S> + Combine<ASTParsec> + Clone + 'a
where
    S: Stream<Item = char>,
{
    a_try(binding())
        .opt()
        .and_left(skip())
        .and(atom())
        .and_left(skip())
        .and(occurrence().opt())
        .and_left(skip())
        .and(additional().opt())
        .and_left(skip())
        .and(transform().opt())
        .fmap(|((((bind, atom), occ), add), trans)| {
            let occ = if occ.is_some() {
                match occ.unwrap() {
                    '?' => POptional(Box::new(atom)),
                    '*' => PRepeat(true, Box::new(atom)),
                    '+' => PRepeat(false, Box::new(atom)),
                    _ => atom,
                }
            } else {
                atom
            };

            let bind = if bind.is_some() {
                PBind(bind.unwrap(), Box::new(occ))
            } else {
                occ
            };

            let add = if add.is_some() {
                let value = add.unwrap();

                if value.0 {
                    PChoice(Box::new(bind), Box::new(value.1))
                } else {
                    PSequence(Box::new(bind), Box::new(value.1))
                }
            } else {
                bind
            };

            let trans = if trans.is_some() {
                PMap(Box::new(add), trans.unwrap())
            } else {
                add
            };

            trans
        })
}

#[inline]
fn binding<'a, S: 'a>() -> impl Parse<String, S> + Combine<String> + Clone + 'a
where
    S: Stream<Item = char>,
{
    ident().and_left(skip()).and_left(char('='))
}

#[inline]
fn occurrence<'a, S: 'a>() -> impl Parse<char, S> + Combine<char> + Clone + 'a
where
    S: Stream<Item = char>,
{
    char_in_set(vec!['+', '?', '*'])
}

fn additional<'a, S: 'a>(
) -> impl Parse<(bool, ASTParsec), S> + Combine<(bool, ASTParsec)> + Clone + 'a
where
    S: Stream<Item = char>,
{
    char('|')
        .opt()
        .fmap(|o| o.is_some())
        .and_left(skip())
        .and(lazy(|| parser(parsec())))
}

#[inline]
fn atom<'a, S: 'a>() -> impl Parse<ASTParsec, S> + Combine<ASTParsec> + Clone + 'a
where
    S: Stream<Item = char>,
{
    char('^')
        .and_left(skip())
        .and_right(atom2())
        .fmap(|p| PNot(Box::new(p)))
        .or(char('!')
            .and_left(skip())
            .and_right(atom2())
            .fmap(|p| PTry(Box::new(p))))
        .or(char('#')
            .and_left(skip())
            .and_right(atom2())
            .fmap(|p| PCheck(Box::new(p))))
        .or(atom2())
}

#[inline]
fn atom2<'a, S: 'a>() -> impl Parse<ASTParsec, S> + Combine<ASTParsec> + Clone + 'a
where
    S: Stream<Item = char>,
{
    ((char('(')
        .and_left(skip())
        .and_right(lazy(|| parser::<'a, _, ASTParsec, S>(parsec())))
        .and_left(skip())
        .and_left(char(')')))
    .or(code().fmap(PCode))
    .or(delimited_char().fmap(PChar))
    .or(delimited_string().fmap(PString))
    .or(ident().fmap(PIdent)))
}

#[inline]
fn transform<'a, S: 'a>() -> impl Parse<String, S> + Combine<String> + Clone + 'a
where
    S: Stream<Item = char>,
{
    string("->").and(skip()).left().and(lazy(code)).right()
}

fn code<'a, S: 'a>() -> impl Parse<String, S> + Combine<String> + Clone + 'a
where
    S: Stream<Item = char>,
{
    char('{')
        .and_right(not_char('}').opt_rep())
        .and_left(char('}'))
        .fmap(|v| v.into_iter().collect())
}

// -------------------------------------------------------------------------------------------------

pub fn celma_parsec<'a, S: 'a>() -> impl Parse<ASTParsec, S> + Combine<ASTParsec> + Clone + 'a
where
    S: Stream<Item = char>,
{
    skip().and_right(parsec()).and_left(skip()).and_left(eos())
}

pub fn celma_parsec_rules<'a, S: 'a>(
) -> impl Parse<Vec<ASTParsecRule>, S> + Combine<Vec<ASTParsecRule>> + Clone + 'a
where
    S: Stream<Item = char>,
{
    skip()
        .and_right(parsec_rules())
        .and_left(skip())
        .and_left(eos())
}
