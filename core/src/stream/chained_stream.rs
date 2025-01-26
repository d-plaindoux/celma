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

use std::marker::PhantomData;

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::position::Position;
use crate::stream::stream::Stream;

pub struct ChainedStream<E, S, L>(Vec<S>, PhantomData<L>, PhantomData<E>)
    where
        S: Stream<Pos = L>,
        L: Position;

impl<E, S, L> ChainedStream<E, S, L>
    where
        S: Stream<Pos = L>,
        L: Position,
{
    #[inline]
    pub fn new(v: Vec<S>) -> Self {
        ChainedStream(v, PhantomData, PhantomData)
    }
}

impl<E, S, L> Clone for ChainedStream<E, S, L>
    where
        S: Stream<Pos = L>,
        L: Position,
{
    fn clone(&self) -> Self {
        ChainedStream::new(vec!(self.0.clone(), self.1.clone()))
    }
}

impl<E, S, L> Stream for ChainedStream<E, S, L>
    where
        S: Stream<Pos = L> + Clone,
        L: Position + Clone,
{
    type Item = E;
    type Pos = L;

    fn position(&self) -> Self::Pos {
        self.1.position()
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        if self.0.is_empty() {
            return (None, self.clone())
        }

        match self.0.next() {
            (Some(a), s) => (Some(a), ChainedStream::new(vec!(s, self.1.clone()))),
            (None, _) =>
                match self.1.next() {
                    (Some(a), s) => (Some(a), ChainedStream::new(vec!(s))),
                    (None, s) => (None, ChainedStream::new(vec!(s)))
                }
        }
    }
}
