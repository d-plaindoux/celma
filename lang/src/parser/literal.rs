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

use crate::parser::ff::{First, Token};
use celma_core::parser::literal::{Chars, StringDelimited};
use celma_core::stream::stream::Stream;

impl<'a, 'b, S> First<S> for Chars<'b>
where
    S: Stream<Item = char>,
{
    fn first(&self) -> Vec<Token<<S as Stream>::Item>> {
        let Self(v) = self;

        vec![Token::Atom(v.chars().next().unwrap())]
    }
}

impl<S> First<S> for StringDelimited
where
    S: Stream<Item = char>,
{
    fn first(&self) -> Vec<Token<S::Item>> {
        vec![Token::Atom('"')]
    }
}

pub fn delimited_string() -> StringDelimited {
    StringDelimited
}
