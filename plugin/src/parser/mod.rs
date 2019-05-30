use celma::parser::and::AndOperation;
use celma::parser::char::{char, char_in_set, number, not_char};
use celma::parser::fmap::FMapOperation;
use celma::parser::parser::{Combine, Parse};
use celma::parser::repeat::RepeatOperation;
use celma::parser::response::Response::{Reject, Success};
use celma::stream::char_stream::CharStream;
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
fn skip<S: 'static>() -> impl Parse<(), S> + Combine<()> + Clone
    where
        S: Stream<Item=char>,
{
    char_in_set(vec!['\n', '\r', '\t', ' '])
        .opt_rep()
        .fmap({ |_| () })
}

#[inline]
fn number<S: 'static>() -> impl Parse<(), S> + Combine<()> + Clone
    where
        S: Stream<Item=char>,
{
    digit().rep().fmap(|_| ())
}

#[inline]
fn delimited_char<S: 'static>() -> impl Parse<(), S> + Combine<()> + Clone
    where
        S: Stream<Item=char>,
{
    char('\'').and(not_char('\'')).and(char('\'')).fmap(|_| ())
}

#[inline]
fn delimited_string<S: 'static>() -> impl Parse<(), S> + Combine<()> + Clone
    where
        S: Stream<Item=char>,
{
    char('\"').and(not_char('\"')).opt_rep().and(char('\"')).fmap(|_| ())
}

// -------------------------------------------------------------------------------------------------
