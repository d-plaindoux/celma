use celma::parser::and::{AndOperation, AndProjection};
use celma::parser::char::{alpha, char, char_in_set, digit, not_char};
use celma::parser::core::{parser, Parser};
use celma::parser::lazy::lazy;
use celma::parser::monadic::FMapOperation;
use celma::parser::or::OrOperation;
use celma::parser::parser::Parse;
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

fn skip<S: 'static>() -> Parser<(), S>
where
    S: Stream<Item = char>,
{
    parser(
        char_in_set(vec!['\n', '\r', '\t', ' '])
            .opt_rep()
            .fmap({ |_| () }),
    )
}

fn number<S: 'static>() -> Parser<Token, S>
where
    S: Stream<Item = char>,
{
    parser(
        digit()
            .rep()
            .fmap(|v| v.into_iter().collect::<String>())
            .fmap(|s| Token::Number(s.parse::<i32>().unwrap())),
    )
}

fn ident<S: 'static>() -> Parser<Token, S>
where
    S: Stream<Item = char>,
{
    parser(
        alpha()
            .rep()
            .fmap(|v| Token::Ident(v.into_iter().collect::<String>())),
    )
}

fn string<S: 'static>() -> Parser<Token, S>
where
    S: Stream<Item = char>,
{
    parser(
        char('"')
            .and(not_char('"').opt_rep())
            .right()
            .and(char('"'))
            .left()
            .fmap(|v| Token::String(v.into_iter().collect::<String>())),
    )
}

fn item<S: 'static>() -> Parser<Token, S>
where
    S: Stream<Item = char>,
{
    parser(number().or(ident()).or(string()).or(record()))
}

fn sequence<S: 'static>() -> Parser<Vec<Token>, S>
where
    S: Stream<Item = char>,
{
    parser(
        item()
            .and(skip())
            .left()
            .and(
                (char(',').and(skip()))
                    .and(item().and(skip()).left())
                    .right()
                    .opt_rep(),
            )
            .fmap(|(e, v)| [vec![e], v].concat()),
    )
}

fn record<S: 'static>() -> Parser<Token, S>
where
    S: Stream<Item = char>,
{
    parser(
        char('[')
            .and(skip())
            .and(lazy(|| sequence()))
            .right()
            .and(char(']').and(skip()))
            .left()
            .fmap(|v| Token::Record(v)),
    )
}

fn main() {
    match number().parse(CharStream::new("123")) {
        Success(Token::Number(ref s), _, _) if *s == 123 => println!("Ident = {}", s),
        _ => println!("KO"),
    }

    match ident().parse(CharStream::new("Toto")) {
        Success(Token::Ident(ref s), _, _) if *s == String::from("Toto") => {
            println!("Ident = {}", s)
        }
        _ => println!("KO"),
    }

    match string().parse(CharStream::new(r#""Toto""#)) {
        Success(Token::String(ref s), _, _) if *s == String::from("Toto") => {
            println!("Ident = {}", s)
        }
        _ => println!("KO"),
    }

    match record().parse(CharStream::new(
        r#"[ "Hello" , 123 , World, [ "Hello" , 123 , World ] ]"#,
    )) {
        Success(Token::Record(ref s), _, _) if s.len() == 3 => println!("Record = {:?}", s),
        _ => println!("KO"),
    }
}
