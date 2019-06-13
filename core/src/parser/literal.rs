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

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;

#[derive(Copy, Clone)]
pub struct Chars<'b>(&'b str);

impl<'a> Combine<&'a str> for Chars<'a> {}

impl<'a, 'b, S> Parse<&'b str, S> for Chars<'b>
where
    S: Stream<Item = char>,
{
    fn parse(&self, s: S) -> Response<&'b str, S> {
        let Self(v) = self;
        let mut index = 0;
        let mut ns = s;

        loop {
            if index == v.len() {
                return Success(v, ns, index > 0);
            }

            match (ns.next(), v.chars().nth(index)) {
                ((Some(c), ref next), Some(v)) if c == v => {
                    index += 1;
                    ns = next.clone();
                }
                _ => {
                    return Reject(ns, false);
                }
            }
        }
    }
}

pub fn string(s: &str) -> Chars {
    Chars(s)
}
