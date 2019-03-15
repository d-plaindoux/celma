use celma::parser::and::{AndOperation, AndProjection};
use celma::parser::char::{alpha, char, char_in_set, digit, not_char};
use celma::parser::monadic::FMapOperation;
use celma::parser::or::OrOperation;
use celma::parser::parser::{Combine, Parse};
use celma::parser::repeat::RepeatOperation;
use celma::parser::response::Response::Success;
use celma::stream::char_stream::CharStream;
use celma::stream::stream::Stream;
use celma::parser::lazy::lazy;

#[derive(Debug, Clone)]
enum Token {
    Number(i32),
    Ident(String),
    String(String),
    Record(Vec<Token>),
}

fn skip<S>() -> impl Combine<()> + Parse<(), S> + Clone
    where
        S: Stream<Item=char>,
{
    char_in_set(vec!['\n', '\r', '\t', ' '])
        .opt_rep()
        .fmap({ |_| () })
}

fn number<S>() -> impl Combine<Token> + Parse<Token, S> + Clone
    where
        S: Stream<Item=char>,
{
    digit()
        .rep()
        .fmap(|v| v.into_iter().collect::<String>())
        .fmap(|s| Token::Number(s.parse::<i32>().unwrap()))
}

fn ident<S>() -> impl Combine<Token> + Parse<Token, S> + Clone
    where
        S: Stream<Item=char>,
{
    alpha()
        .rep()
        .fmap(|v| Token::Ident(v.into_iter().collect::<String>()))
}

fn string<S>() -> impl Combine<Token> + Parse<Token, S> + Clone
    where
        S: Stream<Item=char>,
{
    char('"')
        .and(not_char('"').opt_rep())
        .right()
        .and(char('"'))
        .left()
        .fmap(|v| Token::String(v.into_iter().collect::<String>()))
}

fn item<S>() -> impl Combine<Token> + Parse<Token, S> + Clone
    where
        S: Stream<Item=char>,
{
    number().or(ident()).or(string())
}

fn sequence<P, S>(p: P, s: char) -> impl Combine<Vec<Token>> + Parse<Vec<Token>, S> + Clone
    where
        P: Combine<Token> + Parse<Token, S> + Clone,
        S: Stream<Item=char>,
{
    p.clone()
        .and(skip())
        .left()
        .and(
            (char(s).and(skip()))
                .and(p.clone().and(skip()).left())
                .right()
                .opt_rep(),
        )
        .fmap(|(e, v)| [vec![e], v].concat())
}

fn record<S>() -> impl Combine<Token> + Parse<Token, S> + Clone
    where
        S: Stream<Item=char>,
{
    char('[').and(skip())
        .and(sequence(item(), ',')).right()
        .and(char(']').and(skip())).left()
        .fmap(|v| Token::Record(v))
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

    match record().parse(CharStream::new(r#"[ "Hello" , 123 , World ]"#)) {
        Success(Token::Record(ref s), _, _) if s.len() == 3 => println!("Record = {:?}", s),
        _ => println!("KO"),
    }
}
