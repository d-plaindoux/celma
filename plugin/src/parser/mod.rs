#![allow(dead_code)]

use celma::parser::and::{AndOperation, AndProjection};
use celma::parser::char::{char, char_in_set, digit, not_char};
use celma::parser::fmap::FMapOperation;
use celma::parser::parser::{Combine, Parse};
use celma::parser::repeat::RepeatOperation;
use celma::stream::stream::Stream;

// -------------------------------------------------------------------------------------------------
// Grammar - Parser using Celma ^_^
// -------------------------------------------------------------------------------------------------
//
// parser     ::= binding? atom occurrence? additional? transform?
// binding    ::= IDENT '='
// occurrence ::= ("*" | "+"  | "?")
// additional ::= ("~" | "<~" | "~>" | "|") parser
// transform  ::= "=>" { -- rust code -- }
// atom       ::= '(' parser ')' | CHAR | NUMBER | STRING | ^CHAR | { -- rust code -- }
//
// -------------------------------------------------------------------------------------------------
// Note: Syn should be better but this done for dog-fooding purpose)
// -------------------------------------------------------------------------------------------------

#[inline]
fn skip<'a, S: 'a>() -> impl Parse<(), S> + Combine<()> + Clone
where
    S: Stream<Item = char>,
{
    char_in_set(vec!['\n', '\r', '\t', ' '])
        .opt_rep()
        .fmap({ |_| () })
}

#[inline]
fn delimiter_string<'a, S: 'a>() -> impl Parse<String, S> + Combine<String> + Clone
where
    S: Stream<Item = char>,
{
    char('"')
        .and(not_char('"').opt_rep())
        .right()
        .and(char('"'))
        .left()
        .fmap(|v| v.into_iter().collect::<String>())
}

#[inline]
fn number<'a, S: 'a>() -> impl Parse<i32, S> + Combine<i32> + Clone
where
    S: Stream<Item = char>,
{
    digit()
        .rep()
        .fmap(|v| v.into_iter().collect::<String>())
        .fmap(|s| s.parse::<i32>().unwrap())
}

#[inline]
fn delimited_char<'a, S: 'a>() -> impl Parse<char, S> + Combine<char> + Clone
where
    S: Stream<Item = char>,
{
    char('\'')
        .and(not_char('\''))
        .right()
        .and(char('\''))
        .left()
        .fmap(|c| c)
}

// -------------------------------------------------------------------------------------------------
