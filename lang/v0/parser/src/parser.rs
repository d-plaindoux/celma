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
use celma_core::parser::a_try::a_try;
use celma_core::parser::and::{AndOperation, AndProjection};
use celma_core::parser::bind::BindOperation;
use celma_core::parser::char::{a_char, char_in_range, char_in_set, not_char};
use celma_core::parser::core::{eos, fail, parser, returns};
use celma_core::parser::lazy::lazy;
use celma_core::parser::literal::{delimited_char, delimited_string, string};
use celma_core::parser::map::MapOperation;
use celma_core::parser::option::OptionalOperation;
use celma_core::parser::or::OrOperation;
use celma_core::parser::repeat::RepeatOperation;
use celma_core::parser::specs::{Combine, Parse};
use celma_core::stream::specs::Stream;
use std::ops::Range;

use celma_lang_ast::syntax::ASTParsec::{
    PAtom, PAtoms, PBind, PCheck, PChoice, PCode, PIdent, PLookahead, PMap, PNot, POptional,
    PRepeat, PSequence, PTry,
};
use celma_lang_ast::syntax::{ASTParsec, ASTParsecRule};

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
    (char_in_range(Range {
        start: 'A',
        end: 'Z',
    })
    .or(char_in_range(Range {
        start: 'a',
        end: 'z',
    }))
    .or(char_in_range(Range {
        start: '0',
        end: '9',
    }))
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
fn parsec_rules<'a, S>(
) -> impl Parse<Vec<ASTParsecRule<char>>, S> + Combine<Vec<ASTParsecRule<char>>> + 'a
where
    S: Stream<Item = char> + 'a,
{
    string("let")
        .and_left(skip())
        .and_right(ident())
        .and_left(skip())
        .and(kind().opt())
        .and_left(skip())
        .and_left(a_char(':'))
        .and_left(skip())
        .and(kind())
        .and_left(skip())
        .and_left(string("="))
        .and_left(skip())
        .and(parsec())
        .and_left(skip())
        .map(
            |(((n, i), r), b): (((String, Option<String>), String), ASTParsec<char>)| {
                ASTParsecRule {
                    public: true,
                    name: n,
                    input: i.unwrap_or(String::from("char")),
                    returns: r,
                    rule: Box::new(b),
                }
            },
        )
        .rep()
}

// -------------------------------------------------------------------------------------------------

#[inline]
fn parsec<'a, S>() -> impl Parse<ASTParsec<char>, S> + Combine<ASTParsec<char>> + 'a
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
                    '?' => POptional(Box::new(atom)),
                    '*' => PRepeat(true, Box::new(atom)),
                    '+' => PRepeat(false, Box::new(atom)),
                    _ => atom,
                }
            } else {
                atom
            };

            let bind = if let Some(value) = bind {
                PBind(value, Box::new(occ))
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

            if let Some(value) = trans {
                PMap(Box::new(add), value)
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

fn additional<'a, S>(
) -> impl Parse<(bool, ASTParsec<char>), S> + Combine<(bool, ASTParsec<char>)> + 'a
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
fn atom<'a, S>() -> impl Parse<ASTParsec<char>, S> + Combine<ASTParsec<char>> + 'a
where
    S: Stream<Item = char> + 'a,
{
    a_char('^')
        .and_left(skip())
        .and_right(atom2())
        .map(|p| PNot(Box::new(p)))
        .or(a_char('!')
            .and_left(skip())
            .and_right(atom2())
            .map(|p| PTry(Box::new(p))))
        .or(a_char('#')
            .and_left(skip())
            .and_right(atom2())
            .map(|p| PCheck(Box::new(p))))
        .or(a_char('/')
            .and_left(skip())
            .and_right(atom2())
            .map(|p| PLookahead(Box::new(p))))
        .or(atom2())
}

#[inline]
fn atom2<'a, S>() -> impl Parse<ASTParsec<char>, S> + Combine<ASTParsec<char>> + 'a
where
    S: Stream<Item = char> + 'a,
{
    (a_char('(')
        .and_left(skip())
        .and_right(lazy(|| parser(parsec())))
        .and_left(skip())
        .and_left(a_char(')')))
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

pub fn celma_parsec<'a, S>() -> impl Parse<ASTParsec<char>, S> + Combine<ASTParsec<char>> + 'a
where
    S: Stream<Item = char> + 'a,
{
    skip().and_right(parsec()).and_left(skip()).and_left(eos())
}

pub fn celma_parsec_rules<'a, S>(
) -> impl Parse<Vec<ASTParsecRule<char>>, S> + Combine<Vec<ASTParsecRule<char>>> + 'a
where
    S: Stream<Item = char> + 'a,
{
    skip()
        .and_right(parsec_rules())
        .and_left(skip())
        .and_left(eos())
}
