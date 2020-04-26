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

            let (oc, next) = ns.next();
            let sc = v.chars().nth(index);

            match (oc, sc) {
                (Some(c), Some(v)) if c == v => {
                    index += 1;
                    ns = next;
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

// -------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct StringDelimited;

impl<'a> Combine<String> for StringDelimited {}

impl<S> Parse<String, S> for StringDelimited
where
    S: Stream<Item = char>,
{
    fn parse(&self, s: S) -> Response<String, S> {
        let (c, nsp) = s.next();

        if c.is_none() || c.unwrap() != '"' {
            return Reject(s, false);
        }

        let mut ns = nsp;
        let mut rs = Vec::<char>::new();

        loop {
            let (c, nsp) = ns.next();

            if c.is_none() {
                return Reject(ns, true);
            } else if c.unwrap() == '"' {
                let r: String = rs.into_iter().collect();
                return Success(r, nsp, true);
            } else if c.unwrap() == '\\' {
                let (c, nsp) = nsp.next();
                rs.push('\\');
                rs.push(c.unwrap());
                ns = nsp;
            } else {
                rs.push(c.unwrap());
                ns = nsp;
            }
        }
    }

    fn check(&self, s: S) -> Response<(), S> {
        let (c, nsp) = s.next();

        if c.is_none() || c.unwrap() != '"' {
            return Reject(s, false);
        }

        let mut ns = nsp;

        loop {
            let (c, nsp) = ns.next();

            if c.is_none() {
                return Reject(ns, true);
            } else if c.unwrap() == '"' {
                return Success((), nsp, true);
            } else if c.unwrap() == '\\' {
                ns = nsp.next().1;
            } else {
                ns = nsp;
            }
        }
    }
}

pub fn delimited_string() -> StringDelimited {
    StringDelimited
}
