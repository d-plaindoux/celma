#![allow(dead_code)]

use celma::parser::and::{AndOperation, AndProjection};
use celma::parser::char::{char, char_in_range, char_in_set, not_char};
use celma::parser::core::parser;
use celma::parser::fmap::FMapOperation;
use celma::parser::lazy::lazy;
use celma::parser::literal::string;
use celma::parser::option::OptionalOperation;
use celma::parser::or::OrOperation;
use celma::parser::parser::{Combine, Parse};
use celma::parser::repeat::RepeatOperation;
use celma::stream::stream::Stream;

use crate::parser::ASTParsec::{PBind, PChoice, PCode, PMap, POptional, PRepeat};

// -------------------------------------------------------------------------------------------------
// Grammar - Parser using Celma ^_^
// -------------------------------------------------------------------------------------------------
//
// parsec     ::= binding? atom occurrence? additional? transform?
// binding    ::= IDENT '='
// occurrence ::= ("*" | "+"  | "?")
// additional ::= "|"? parsec
// transform  ::= "=>" { -- rust code -- }
// atom       ::= '(' parsec ')' | { -- rust code -- }
//
// -------------------------------------------------------------------------------------------------
// Note: Syn should be better but this done for dog-fooding purpose)
// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum ASTParsec {
    PSequence(Box<ASTParsec>, Box<ASTParsec>),
    PChoice(Box<ASTParsec>, Box<ASTParsec>),
    POptional(Box<ASTParsec>),
    PRepeat(bool, Box<ASTParsec>),
    PBind(String, Box<ASTParsec>),
    PCode(String),
    PMap(Box<ASTParsec>, String),
}

#[inline]
fn skip<'a, S: 'a>() -> impl Parse<(), S> + Combine<()> + Clone
where
    S: Stream<Item = char>,
{
    char_in_set(vec!['\n', '\r', '\t', ' '])
        .opt_rep()
        .fmap({ |_| () })
}

// -------------------------------------------------------------------------------------------------

#[inline]
pub fn parsec<'a, S: 'a>() -> impl Parse<ASTParsec, S> + Combine<ASTParsec> + Clone + 'a
where
    S: Stream<Item = char>,
{
    binding()
        .opt()
        .and(skip())
        .left()
        .and(atom())
        .and(skip())
        .left()
        .and(occurrence().opt())
        .and(skip())
        .left()
        .and(additional().opt())
        .and(skip())
        .left()
        .and(transform().opt())
        .fmap(|((((bind, atom), occ), add), trans)| {
            let bind = if bind.is_some() {
                PBind(bind.unwrap(), Box::new(atom))
            } else {
                atom
            };

            let occ = if occ.is_some() {
                match occ.unwrap() {
                    '?' => POptional(Box::new(bind)),
                    '*' => PRepeat(true, Box::new(bind)),
                    '+' => PRepeat(false, Box::new(bind)),
                    _ => bind,
                }
            } else {
                bind
            };

            let add = if add.is_some() {
                let value = add.unwrap();

                if value.0 {
                    PChoice(Box::new(occ), Box::new(value.1))
                } else {
                    occ
                }
            } else {
                occ
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
    let letter = char_in_range('A'..'Z').or(char_in_range('a'..'z'));

    letter
        .rep()
        .and(skip())
        .left()
        .and(char('='))
        .left()
        .fmap(|v| v.into_iter().collect())
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
        .and(skip())
        .left()
        .and(lazy(|| parser(parsec())))
}

#[inline]
fn atom<'a, S: 'a>() -> impl Parse<ASTParsec, S> + Combine<ASTParsec> + Clone + 'a
where
    S: Stream<Item = char>,
{
    char('(')
        .and(skip())
        .left()
        .and(lazy(|| parser::<'a, _, ASTParsec, S>(parsec())))
        .right()
        .and(skip())
        .left()
        .and(char(')'))
        .left()
        .or(code().fmap(PCode))
}

#[inline]
fn transform<'a, S: 'a>() -> impl Parse<String, S> + Combine<String> + Clone + 'a
where
    S: Stream<Item = char>,
{
    string("=>").and(skip()).left().and(lazy(code)).right()
}

fn code<'a, S: 'a>() -> impl Parse<String, S> + Combine<String> + Clone + 'a
where
    S: Stream<Item = char>,
{
    char('{')
        .and(not_char('}').opt_rep())
        .right()
        .and(char('}'))
        .left()
        .fmap(|v| v.into_iter().collect())
}

// -------------------------------------------------------------------------------------------------
