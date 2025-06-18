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
use celma_v0_core::parser::a_try::a_try;
use celma_v0_core::parser::and::{AndOperation, AndProjection};
use celma_v0_core::parser::bind::BindOperation;
use celma_v0_core::parser::char::{a_char, char_in_range, char_in_set, not_char};
use celma_v0_core::parser::core::{eos, fail, parser, returns};
use celma_v0_core::parser::lazy::lazy;
use celma_v0_core::parser::literal::{delimited_char, delimited_string, string};
use celma_v0_core::parser::map::MapOperation;
use celma_v0_core::parser::option::OptionalOperation;
use celma_v0_core::parser::or::OrOperation;
use celma_v0_core::parser::repeat::RepeatOperation;
use celma_v0_core::parser::specs::{Combine, Parse};
use celma_v0_core::stream::specs::Stream;

use celma_v0_ast::syntax::ASTParsec::{
    PAtom, PAtoms, PBind, PCheck, PChoice, PCode, PEpsilon, PIdent, PMap, PNot, POptional, PRepeat,
    PSequence, PTry,
};
use celma_v0_ast::syntax::{ASTParsec, ASTParsecRule};

#[inline]
fn skip<'a, S>() -> impl Parse<(), S> + Combine<()> + 'a
where
    S: Stream<Item = char> + 'a,
{
    char_in_set(vec!['\n', '\r', '\t', ' '])
        .opt_rep()
        .map(|_| ())
}

#[inline]
fn ident<'a, S>() -> impl Parse<String, S> + Combine<String> + 'a
where
    S: Stream<Item = char> + 'a,
{
    (char_in_range('A'..='Z')
        .or(char_in_range('a'..='z'))
        .or(char_in_range('0'..='9'))
        .or(a_char('_')))
    .rep()
    .map(|v| v.into_iter().collect())
    .bind(|s| {
        if s == *"let" {
            parser(fail(false))
        } else {
            parser(returns(s))
        }
    })
}

// -------------------------------------------------------------------------------------------------

#[inline]
fn parsec_rules<'a, S>() -> impl Parse<Vec<ASTParsecRule>, S> + Combine<Vec<ASTParsecRule>> + 'a
where
    S: Stream<Item = char> + 'a,
{
    string("let")
        .and_left(skip())
        .and_right(ident())
        .and_left(skip())
        .and(kind().opt())
        .and_left(skip())
        .and(a_char(':').and_right(skip()).and_right(kind()).opt())
        .and_left(skip())
        .and_left(string("="))
        .and_left(skip())
        .and(parsec())
        .and_left(skip())
        .map(
            |(((n, i), r), b): (((String, Option<String>), Option<String>), ASTParsec)| {
                ASTParsecRule {
                    name: n,
                    input: i.unwrap_or(String::from("char")),
                    returns: r.unwrap_or(String::from("()")),
                    rule: b,
                }
            },
        )
        .rep()
}

// -------------------------------------------------------------------------------------------------

#[inline]
fn parsec<'a, S>() -> impl Parse<ASTParsec, S> + Combine<ASTParsec> + 'a
where
    S: Stream<Item = char> + 'a,
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
        .map(|((((bind, atom), occ), add), trans)| {
            let occ = if let Some(value) = occ {
                match value {
                    '?' => POptional(atom.wrap()),
                    '*' => PRepeat(true, atom.wrap()),
                    '+' => PRepeat(false, atom.wrap()),
                    _ => atom,
                }
            } else {
                atom
            };

            let bind = if let Some(value) = bind {
                PBind(value, occ.wrap())
            } else {
                occ
            };

            let add = if add.is_some() {
                let value = add.unwrap();

                if value.0 {
                    PChoice(bind.wrap(), value.1.wrap())
                } else {
                    PSequence(bind.wrap(), value.1.wrap())
                }
            } else {
                bind
            };

            if let Some(value) = trans {
                PMap(add.wrap(), value)
            } else {
                add
            }
        })
}

#[inline]
fn binding<'a, S>() -> impl Parse<String, S> + Combine<String> + 'a
where
    S: Stream<Item = char> + 'a,
{
    ident().and_left(skip()).and_left(a_char('='))
}

#[inline]
fn occurrence<'a, S>() -> impl Parse<char, S> + Combine<char> + 'a
where
    S: Stream<Item = char> + 'a,
{
    char_in_set(vec!['+', '?', '*'])
}

fn additional<'a, S>() -> impl Parse<(bool, ASTParsec), S> + Combine<(bool, ASTParsec)> + 'a
where
    S: Stream<Item = char> + 'a,
{
    a_char('|')
        .opt()
        .map(|o| o.is_some())
        .and_left(skip())
        .and(lazy(|| parser(parsec())))
}

#[inline]
fn atom<'a, S>() -> impl Parse<ASTParsec, S> + Combine<ASTParsec> + 'a
where
    S: Stream<Item = char> + 'a,
{
    a_char('^')
        .and_left(skip())
        .and_right(atom2())
        .map(|p| PNot(p.wrap()))
        .or(a_char('!')
            .and_left(skip())
            .and_right(atom2())
            .map(|p| PTry(p.wrap())))
        .or(a_char('#')
            .and_left(skip())
            .and_right(atom2())
            .map(|p| PCheck(p.wrap())))
        .or(atom2())
}

#[inline]
fn atom2<'a, S>() -> impl Parse<ASTParsec, S> + Combine<ASTParsec> + 'a
where
    S: Stream<Item = char> + 'a,
{
    a_char('(')
        .and_left(skip())
        .and_right(lazy(|| parser(parsec())).opt())
        .map(|p| p.unwrap_or_else(PEpsilon))
        .and_left(skip())
        .and_left(a_char(')'))
        .or(code().map(PCode))
        .or(delimited_char().map(PAtom))
        .or(delimited_string().map(|l| PAtoms(l.chars().collect())))
        .or(ident().map(PIdent))
}

#[inline]
fn transform<'a, S>() -> impl Parse<String, S> + Combine<String> + 'a
where
    S: Stream<Item = char> + 'a,
{
    string("->").and(skip()).left().and(lazy(code)).right()
}

fn kind<'a, S>() -> impl Parse<String, S> + Combine<String> + 'a
where
    S: Stream<Item = char> + 'a,
{
    a_char('{')
        .and_right(not_char('}').opt_rep())
        .and_left(a_char('}'))
        .map(|v| v.into_iter().collect())
}

fn code<'a, S>() -> impl Parse<String, S> + Combine<String> + 'a
where
    S: Stream<Item = char> + 'a,
{
    a_char('{')
        .and_right(not_char('}').opt_rep())
        .and_left(a_char('}'))
        .map(|v| v.into_iter().collect())
}

// -------------------------------------------------------------------------------------------------

pub fn celma_parsec<'a, S>() -> impl Parse<ASTParsec, S> + Combine<ASTParsec> + 'a
where
    S: Stream<Item = char> + 'a,
{
    skip().and_right(parsec()).and_left(skip()).and_left(eos())
}

pub fn celma_parsec_rules<'a, S>()
-> impl Parse<Vec<ASTParsecRule>, S> + Combine<Vec<ASTParsecRule>> + 'a
where
    S: Stream<Item = char> + 'a,
{
    skip()
        .and_right(parsec_rules())
        .and_left(skip())
        .and_left(eos())
}
