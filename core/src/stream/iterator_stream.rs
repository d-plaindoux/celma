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

use std::iter::Iterator;
use std::marker::PhantomData;

use crate::stream::end_line::EndLine;
use crate::stream::position::Position;
use crate::stream::stream::Len;
use crate::stream::stream::Stream;

#[derive(Clone)]
pub struct IteratorStream<E, I, P>(I, P, PhantomData<E>)
where
    I: Iterator<Item = E>,
    E: EndLine,
    P: Position;

impl<'a, E, I> IteratorStream<E, I, (usize, usize, usize)>
where
    I: Iterator<Item = E>,
    E: EndLine,
{
    pub fn new(s: I) -> Self {
        IteratorStream(s, <(usize, usize, usize)>::new(), PhantomData)
    }
}

impl<'a, E, I, P> IteratorStream<E, I, P>
where
    I: Iterator<Item = E>,
    E: EndLine,
    P: Position,
{
    pub fn new_with_position(s: I, p: P) -> Self {
        IteratorStream(s, p, PhantomData)
    }
}

impl<E, I, P> Stream for IteratorStream<E, I, P>
where
    I: Iterator<Item = E> + Clone,
    E: EndLine + Clone,
    P: Position + Clone,
{
    type Item = E;
    type Pos = P;

    fn position(&self) -> Self::Pos {
        self.1.clone()
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        let mut this = self.clone(); // Mutability required by the Iterator::next call (below)
        let option = this.0.next();

        if option.is_some() {
            (
                option.clone(),
                IteratorStream(
                    this.0,
                    this.1.step(option.unwrap().is_end_line()),
                    PhantomData,
                ),
            )
        } else {
            (option, IteratorStream(this.0, this.1, PhantomData))
        }
    }
}

impl<E, I, P> Len for IteratorStream<E, I, P>
where
    I: Iterator<Item = E> + Clone,
    E: EndLine,
    P: Position,
{
    fn len(&self) -> usize {
        self.0.clone().count()
    }
}
