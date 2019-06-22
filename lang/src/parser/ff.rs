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

use celma_core::stream::stream::Stream;

pub enum Token<E> {
    Atom(E),
    NotAtom(E),
    NoAtom,
    AllAtom,
}

impl<E> Token<E>
where
    E: Clone,
{
    pub fn negate(&self) -> Self {
        match self {
            Token::NoAtom => Token::AllAtom,
            Token::AllAtom => Token::NoAtom,
            Token::NotAtom(v) => Token::Atom(v.clone()),
            Token::Atom(v) => Token::NotAtom(v.clone()),
        }
    }
}

pub trait HasLambda {
    fn has_lambda(&self) -> bool;
}

impl<E> HasLambda for Vec<Token<E>> {
    fn has_lambda(&self) -> bool {
        self.iter()
            .filter(|t| match t {
                Token::NoAtom => true,
                _ => false,
            })
            .next()
            .is_some()
    }
}

pub trait First<S>
where
    S: Stream,
{
    fn first(&self) -> Vec<Token<S::Item>>;
}

pub trait Tokenize<I> {
    fn tokenize(&self) -> Vec<Token<I>>;
}

impl Tokenize<char> for char {
    fn tokenize(&self) -> Vec<Token<char>> {
        vec![Token::Atom(self.clone())]
    }
}

impl Tokenize<char> for Vec<char> {
    fn tokenize(&self) -> Vec<Token<char>> {
        self.iter().map(|&c| Token::Atom(c)).collect::<Vec<_>>()
    }
}

/*
impl Tokenize<char> for Range<char> {
    fn tokenize(&self) -> Vec<Token<char>> {
        self.collect::<Vec<char>>().tokenize()
    }
}
*/
