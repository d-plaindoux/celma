/*
   Copyright 2019 Didier Plaindoux

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

use std::ops::Range;

use crate::parser::ff::{First, Token};
use crate::parser::or::OrOperation;
use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::satisfy::Satisfy;
use crate::stream::stream::Stream;

pub trait Tokenize<I> {
    fn tokenize(&self) -> Vec<Token<I>>;
}

impl<E> Tokenize<E> for () {
    fn tokenize(&self) -> Vec<Token<E>> {
        vec![Token::AllAtom]
    }
}

impl Tokenize<char> for char {
    fn tokenize(&self) -> Vec<Token<char>> {
        vec![Token::Atom(*self)]
    }
}

impl Tokenize<char> for Vec<char> {
    fn tokenize(&self) -> Vec<Token<char>> {
        self.iter().map(|&c| Token::Atom(c)).collect::<Vec<_>>()
    }
}

impl Tokenize<char> for Range<char> {
    fn tokenize(&self) -> Vec<Token<char>> {
        vec![Token::NoAtom] // TODO
    }
}

// -------------------------------------------------------------------------------------------------

#[inline]
pub fn char<S>(c: char) -> impl First<S> + Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    Satisfy::new(c, |&v, &c| v == c)
}

#[inline]
pub fn not_char<S>(c: char) -> impl First<S> + Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    Satisfy::new(c, |&v, &c| v != c)
}

#[inline]
pub fn char_in_range<S>(r: Range<char>) -> impl First<S> + Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    Satisfy::new(r, |&v, r| r.start <= v && v <= r.end)
}

#[inline]
pub fn char_in_set<S>(r: Vec<char>) -> impl First<S> + Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    Satisfy::new(r, |v, r| r.contains(v))
}

#[inline]
pub fn digit<S>() -> impl First<S> + Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    char_in_range('0'..'9')
}

#[inline]
pub fn alpha_lower<S>() -> impl First<S> + Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    char_in_range('a'..'z')
}

#[inline]
pub fn alpha_upper<S>() -> impl First<S> + Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    char_in_range('A'..'Z')
}

#[inline]
pub fn alpha<S>() -> impl First<S> + Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    alpha_lower().or(alpha_upper())
}

#[inline]
pub fn space<S>() -> impl First<S> + Parse<char, S> + Combine<char>
where
    S: Stream<Item = char>,
{
    char_in_set(vec![' ', '\t', '\r', '\n'])
}
