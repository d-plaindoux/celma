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

use crate::stream::position::Position;
use crate::stream::stream::Len;
use crate::stream::stream::Stream;

#[derive(Clone)]
pub struct CharStream<'a, P>(&'a str, P)
where
    P: Position;

impl<'a> CharStream<'a, (usize, usize, usize)> {
    pub fn new(v: &'a str) -> Self {
        Self::new_with_position(v, <(usize, usize, usize)>::new())
    }
}

impl<'a, P> CharStream<'a, P>
where
    P: Position,
{
    pub fn new_with_position(v: &'a str, p: P) -> Self {
        Self(v, p)
    }
}

impl<'a, P> Stream for CharStream<'a, P>
where
    P: Position + Clone,
{
    type Item = char;
    type Pos = P;

    fn position(&self) -> Self::Pos {
        self.1.clone()
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        let option = self.0.chars().next();

        if option.is_some() {
            let np = self.1.step(option.unwrap() == '\n');

            (
                option,
                CharStream(self.0.get(1..self.0.len()).unwrap_or(""), np),
            )
        } else {
            (None, CharStream(self.0, self.1.clone()))
        }
    }
}

impl<'a, P> Len for CharStream<'a, P>
where
    P: Position,
{
    fn len(&self) -> usize {
        self.0.len()
    }
}
