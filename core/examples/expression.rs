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

use celma_core::parser::and::{AndOperation, AndProjection};
use celma_core::parser::char::{alpha, a_char, char_in_set, digit, not_char};
use celma_core::parser::core::{eos, parser};
use celma_core::parser::fmap::FMapOperation;
use celma_core::parser::lazy::lazy;
use celma_core::parser::or::OrOperation;
use celma_core::parser::parser::{Combine, Parse};
use celma_core::parser::repeat::RepeatOperation;
use celma_core::parser::response::Response::Success;
use celma_core::stream::char_stream::CharStream;
use celma_core::stream::stream::Stream;

#[derive(Debug, Clone)]
enum Token {
    Number(i32),
    Ident(String),
    String(String),
    Record(Vec<Token>),
}

#[inline]
fn skip<'a, S: 'a>() -> impl Parse<(), S> + Combine<()> + 'a
where
    S: Stream<Item = char>,
{
    char_in_set(vec!['\n', '\r', '\t', ' '])
        .opt_rep()
        .fmap(|_| ())
}

#[inline]
fn number<'a, S: 'a>() -> impl Parse<Token, S> + Combine<Token> + 'a
where
    S: Stream<Item = char>,
{
    digit()
        .rep()
        .fmap(|v| v.into_iter().collect::<String>())
        .fmap(|s| Token::Number(s.parse::<i32>().unwrap()))
}

#[inline]
fn ident<'a, S: 'a>() -> impl Parse<Token, S> + Combine<Token> + 'a
where
    S: Stream<Item = char>,
{
    alpha()
        .rep()
        .fmap(|v| Token::Ident(v.into_iter().collect::<String>()))
}

#[inline]
fn string<'a, S: 'a>() -> impl Parse<Token, S> + Combine<Token> + 'a
where
    S: Stream<Item = char>,
{
    a_char('"')
        .and(not_char('"').opt_rep())
        .right()
        .and(a_char('"'))
        .left()
        .fmap(|v| Token::String(v.into_iter().collect::<String>()))
}

#[inline]
fn item<'a, S: 'a>() -> impl Parse<Token, S> + Combine<Token> + 'a
where
    S: Stream<Item = char>,
{
    parser(number().or(ident()).or(string()).or(lazy(|| record())))
}

fn sequence<'a, A: 'a, P: 'a, S: 'a>(p: P, s: char) -> impl Parse<Vec<A>, S> + Combine<Vec<A>> + 'a
where
    A: Clone,
    P: Combine<A> + Parse<A, S>,
    S: Stream<Item = char>,
{
    let p = parser(p);

    p.clone()
        .and(skip())
        .left()
        .and(
            (a_char(s).and(skip()))
                .and(p.and(skip()).left())
                .right()
                .opt_rep(),
        )
        .fmap(|(e, v)| [vec![e], v].concat())
}

#[inline]
fn record<'a, S: 'a>() -> impl Parse<Token, S> + Combine<Token> + 'a
where
    S: Stream<Item = char>,
{
    a_char('[')
        .and(skip())
        .and(sequence(item(), ','))
        .right()
        .and(a_char(']').and(skip()))
        .left()
        .fmap(|v| Token::Record(v))
}

fn main() {
    match number().and(eos()).left().parse(CharStream::new("123")) {
        Success(Token::Number(ref s), _, _) if *s == 123 => println!("Ident = {}", s),
        _ => println!("KO"),
    }

    match ident().and(eos()).left().parse(CharStream::new("Toto")) {
        Success(Token::Ident(ref s), _, _) if *s == String::from("Toto") => {
            println!("Ident = {}", s)
        }
        _ => println!("KO"),
    }

    match string()
        .and(eos())
        .left()
        .parse(CharStream::new(r#""Toto""#))
    {
        Success(Token::String(ref s), _, _) if *s == String::from("Toto") => {
            println!("Ident = {}", s)
        }
        _ => println!("KO"),
    }

    match record().and(eos()).left().parse(CharStream::new(
        r#"[ "Hello" , 123 , World , [ "Hello" , 123 , World ] ]"#,
    )) {
        Success(Token::Record(ref s), _, _) => println!("Record = {:?}", s),
        _ => println!("KO"),
    }
}
