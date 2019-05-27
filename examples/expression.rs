use celma::parser::and::{AndOperation, AndProjection};
use celma::parser::char::{alpha, char, char_in_set, digit, not_char};
use celma::parser::core::{eos, parser};
use celma::parser::fmap::FMapOperation;
use celma::parser::lazy::lazy;
use celma::parser::or::OrOperation;
use celma::parser::parser::{Combine, Parse};
use celma::parser::repeat::RepeatOperation;
use celma::parser::response::Response::Success;
use celma::stream::char_stream::CharStream;
use celma::stream::stream::Stream;

#[derive(Debug, Clone)]
enum Token {
    Number(i32),
    Ident(String),
    String(String),
    Record(Vec<Token>),
}

#[inline]
fn skip<S: 'static>() -> impl Parse<(), S> + Combine<()> + Clone
where
    S: Stream<Item = char>,
{
    char_in_set(vec!['\n', '\r', '\t', ' '])
        .opt_rep()
        .fmap({ |_| () })
}

#[inline]
fn number<S: 'static>() -> impl Parse<Token, S> + Combine<Token> + Clone
where
    S: Stream<Item = char>,
{
    digit()
        .rep()
        .fmap(|v| v.into_iter().collect::<String>())
        .fmap(|s| Token::Number(s.parse::<i32>().unwrap()))
}

#[inline]
fn ident<'a, S: 'a>() -> impl Parse<Token, S> + Combine<Token> + Clone
where
    S: Stream<Item = char>,
{
    alpha()
        .rep()
        .fmap(|v| Token::Ident(v.into_iter().collect::<String>()))
}

#[inline]
fn string<S: 'static>() -> impl Parse<Token, S> + Combine<Token> + Clone
where
    S: Stream<Item = char>,
{
    char('"')
        .and(not_char('"').opt_rep())
        .right()
        .and(char('"'))
        .left()
        .fmap(|v| Token::String(v.into_iter().collect::<String>()))
}

#[inline]
fn item<S: 'static>() -> impl Parse<Token, S> + Combine<Token> + Clone
where
    S: Stream<Item = char>,
{
    number()
        .or(ident())
        .or(string())
        .or(lazy(|| parser(record())))
}

fn sequence<A: 'static, P, S: 'static>(
    p: P,
    s: char,
) -> impl Combine<Vec<A>> + Parse<Vec<A>, S> + Clone
where
    A: Clone,
    P: Combine<A> + Parse<A, S> + Clone,
    S: Stream<Item = char>,
{
    p.clone()
        .and(skip())
        .left()
        .and(
            (char(s).and(skip()))
                .and(p.and(skip()).left())
                .right()
                .opt_rep(),
        )
        .fmap(|(e, v)| [vec![e], v].concat())
}

#[inline]
fn record<S: 'static>() -> impl Parse<Token, S> + Combine<Token> + Clone
where
    S: Stream<Item = char>,
{
    char('[')
        .and(skip())
        .and(sequence(item(), ','))
        .right()
        .and(char(']').and(skip()))
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
