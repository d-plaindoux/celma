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

use std::marker::PhantomData;

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::{Position, Stream};

pub struct ParserStream<'a, P, A, S, L>(&'a P, S, PhantomData<A>, PhantomData<L>)
where
    P: Combine<A> + Parse<A, S>,
    S: Stream<Pos = L>,
    L: Position;

impl<'a, P, A, S, L> ParserStream<'a, P, A, S, L>
where
    P: Combine<A> + Parse<A, S>,
    S: Stream<Pos = L>,
    L: Position,
{
    #[inline]
    pub fn new(p: &'a P, s: S) -> Self {
        ParserStream(p, s, PhantomData, PhantomData)
    }
}

impl<'a, P, A, S, L> Clone for ParserStream<'a, P, A, S, L>
where
    P: Combine<A> + Parse<A, S>,
    S: Stream<Pos=L>,
    L: Position
{
    fn clone(&self) -> Self {
        ParserStream(self.0, self.1.clone(), PhantomData, PhantomData)
    }
}

impl<'a, P, A, S, L> Stream for ParserStream<'a, P, A, S, L>
where
    P: Combine<A> + Parse<A, S>,
    S: Stream<Pos = L>,
    L: Position,
{
    type Item = A;
    type Pos = L;

    fn position(&self) -> Self::Pos {
        self.1.position()
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        match self.0.parse(self.1.clone()) {
            Success(a, s, _) => (Some(a), ParserStream::new(self.0, s.clone())),
            Reject(_, _) => (None, ParserStream::new(self.0, self.1.clone())),
        }
    }
}
