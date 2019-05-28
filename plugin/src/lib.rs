extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

use celma::parser::and::{AndOperation};
use celma::parser::char::{char, char_in_set, not_char};
use celma::parser::fmap::FMapOperation;
use celma::parser::parser::{Combine, Parse};
use celma::parser::repeat::RepeatOperation;
use celma::parser::response::Response::{Success, Reject};
use celma::stream::char_stream::CharStream;
use celma::stream::stream::Stream;

// -------------------------------------------------------------------------------------------------
// Grammar - Parser using Celma ^_^
// -------------------------------------------------------------------------------------------------
//
// s0         ::= parser
//
// parser     ::= atom ("*" | "+"  | "?")? additional? transform?
//
// additional ::= (("~" | "<~" | "~>" | "|") parser)?
// transform  ::= ("fmap" | "bind") { --rust code-- }
//
// atom       ::= '(' parser ')'
//              | CHAR
//              | NUMBER
//              | STRING
//
// -------------------------------------------------------------------------------------------------
// Note: Syn should be better but this done for dog-fooding purpose)
// -------------------------------------------------------------------------------------------------

#[proc_macro]
pub fn parsec<'a>(input: TokenStream) -> TokenStream {
    input
}