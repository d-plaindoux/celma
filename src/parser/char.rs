use std::ops::Range;

use crate::parser::or::OrOperation;
use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::satisfy::Satisfy;
use crate::stream::stream::Stream;

#[inline]
pub fn char<S>(c: char) -> impl Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    Satisfy::new(move |&v| v == c)
}

#[inline]
pub fn not_char<S>(c: char) -> impl Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    Satisfy::new(move |&v| v != c)
}

#[inline]
pub fn char_in<S>(r: Range<char>) -> impl Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    Satisfy::new(move |&v| r.start <= v && v <= r.end)
}

#[inline]
pub fn digit<S>() -> impl Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    char_in('0'..'9')
}

#[inline]
pub fn alpha_lower<S>() -> impl Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    char_in('a'..'z')
}

#[inline]
pub fn alpha_upper<S>() -> impl Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    char_in('A'..'Z')
}

#[inline]
pub fn alpha<S>() -> impl Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    alpha_lower().or(alpha_upper())
}
