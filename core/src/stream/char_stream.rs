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

#![allow(dead_code)]

use crate::stream::stream::Len;
use crate::stream::stream::Stream;

#[derive(Clone)]
pub struct CharStream<'a>(&'a str, usize);

impl<'a> CharStream<'a> {
    pub fn new(s: &'a str) -> CharStream<'a> {
        CharStream(s, 0)
    }
}

impl<'a> Stream for CharStream<'a> {
    type Item = char;

    fn position(&self) -> usize {
        self.1
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        let option = self.0.chars().next();

        if option.is_some() {
            (
                option,
                CharStream(self.0.get(1..self.0.len()).unwrap_or(""), self.1 + 1),
            )
        } else {
            (None, CharStream(self.0, self.1))
        }
    }
}

impl<'a> Len for CharStream<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
