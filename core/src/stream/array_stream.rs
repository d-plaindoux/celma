/*
   Copyright 2019-2020 Didier Plaindoux

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

use crate::stream::end_line::EndLine;
use crate::stream::position::Position;
use crate::stream::stream::Len;
use crate::stream::stream::Stream;

#[derive(Copy, Clone)]
pub struct ArrayStream<'a, A, P>(&'a [A], P)
where
    A: EndLine,
    P: Position;

impl<'a, A> ArrayStream<'a, A, (usize, usize, usize)>
where
    A: EndLine,
{
    pub fn new(v: &'a [A]) -> Self {
        Self::new_with_position(v, <(usize, usize, usize)>::new())
    }
}

impl<'a, A, P> ArrayStream<'a, A, P>
where
    A: EndLine,
    P: Position,
{
    pub fn new_with_position(v: &'a [A], p: P) -> Self {
        Self(v, p)
    }
}

impl<'a, A, P> Stream for ArrayStream<'a, A, P>
where
    A: EndLine + Clone,
    P: Position + Clone,
{
    type Item = A;
    type Pos = P;

    fn position(&self) -> Self::Pos {
        self.1.clone()
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        let option = self.0.get(self.1.offset());

        if option.is_some() {
            (
                option.cloned(),
                ArrayStream(self.0, self.1.step(option.unwrap().is_end_line())),
            )
        } else {
            (option.cloned(), ArrayStream(self.0, self.1.clone()))
        }
    }
}

impl<'a, A, P> Len for ArrayStream<'a, A, P>
where
    A: EndLine,
    P: Position,
{
    fn len(&self) -> usize {
        self.0.len()
    }
}
