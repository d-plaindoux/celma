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

use crate::stream::stream::Stream;
use crate::stream::stream::{Len, Position};

#[derive(Clone)]
pub struct U8Stream<'a>(&'a [u8], usize);

impl<'a> U8Stream<'a> {
    pub fn new(s: &'a [u8]) -> U8Stream<'a> {
        U8Stream(s, 0)
    }
}

impl<'a> Stream for U8Stream<'a> {
    type Item = u8;

    fn position(&self) -> Position {
        Position {
            line: 0,
            char: self.1,
        }
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        if self.1 < self.0.len() {
            (Some(self.0[self.1]), U8Stream(self.0, self.1 + 1))
        } else {
            (None, U8Stream(self.0, self.1))
        }
    }
}

impl<'a> Len for U8Stream<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
