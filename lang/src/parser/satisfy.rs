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

use celma_core::parser::parser::{Combine, Parse};
use celma_core::parser::satisfy::Satisfy;
use celma_core::stream::stream::Stream;

use crate::parser::ff::{First, Token};
use crate::parser::chars::Tokenize;

impl<A, I, S, C> First<S> for Satisfy<A, I, C>
where
    A: Fn(&I, &C) -> bool,
    S: Stream<Item = I>,
    C: Tokenize<I>,
{
    fn first(&self) -> Vec<Token<I>> {
        let Self(c, _, _, _) = self;

        c.tokenize()
    }
}

#[inline]
pub fn not<S, E>(c: E) -> impl Parse<E, S> + Combine<E>
where
    E: Eq + Copy,
    S: Stream<Item = E>,
{
    Satisfy::new(c, |&v, &c| v != c)
}

#[inline]
pub fn eq<S, E>(c: E) -> impl Parse<E, S> + Combine<E>
where
    E: Eq + Copy,
    S: Stream<Item = E>,
{
    Satisfy::new(c, |&v, &c| v == c)
}
