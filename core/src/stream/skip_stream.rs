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
pub struct SkipStream<'a>(&'a str, usize, usize);

impl<'a> SkipStream<'a> {
    pub fn new(s: &'a str) -> SkipStream<'a> {
        SkipStream(s, 1, 0)
    }
}

impl<'a> Stream for SkipStream<'a> {
    type Item = char;

    fn position(&self) -> Position {
        Position {
            line: self.1,
            char: self.2,
        }
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        let option = self.0.chars().next();

        if option.is_some() {
            let (line, char) = if option.unwrap() == '\n' {
                (self.1 + 1, 0)
            } else {
                (self.1, self.2 + 1)
            };

            if vec!(' ','\t','\r','\n').contains(&option.unwrap()) {
                SkipStream(self.0.get(1..self.0.len()).unwrap_or(""), line, char).next()
            } else {
                (
                    option,
                    SkipStream(self.0.get(1..self.0.len()).unwrap_or(""), line, char),
                )
            }
        } else {
            (None, SkipStream(self.0, self.1, self.2))
        }
    }
}

impl<'a> Len for SkipStream<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
