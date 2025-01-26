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

use crate::parser::and::AndOperation;
use crate::parser::char::{char, not_char};
use crate::parser::fmap::FMapOperation;
use crate::parser::or::OrOperation;
use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::repeat::RepeatOperation;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;

// -------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Chars<'b>(&'b str);

impl<'a> Combine<&'a str> for Chars<'a> {}

impl<'a, 'b, S> Parse<&'b str, S> for Chars<'b>
    where
        S: Stream<Item=char>,
{
    fn parse(&self, s: S) -> Response<&'b str, S> {
        let Self(v) = self;
        let mut index = 0;
        let mut ns = s.clone();

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
                    return Reject(s, false);
                }
            }
        }
    }
}

pub fn string(s: &str) -> Chars {
    Chars(s)
}

// -------------------------------------------------------------------------------------------------

pub fn escaped<'a, S: 'a>() -> impl Parse<char, S> + Combine<char> + 'a
    where
        S: Stream<Item=char>,
{
    string(r#"\'"#).fmap(|_| '\'')
        .or(string(r#"\""#).fmap(|_| '\"'))
        .or(string(r#"\\"#).fmap(|_| '\\'))
        .or(string(r#"\n"#).fmap(|_| '\n'))
        .or(string(r#"\r"#).fmap(|_| '\r'))
        .or(string(r#"\t"#).fmap(|_| '\t'))
        .or(string(r#"\0"#).fmap(|_| '\0'))
    // etc. TODO
}

// -------------------------------------------------------------------------------------------------

pub fn delimited_string<'a, S: 'a>() -> impl Parse<String, S> + Combine<String> + 'a
    where
        S: Stream<Item=char>,
{
    char('"')
        .and_right(escaped().or(not_char('"')).opt_rep())
        .and_left(char('"'))
        .fmap(|v| v.into_iter().collect::<String>())
}

// -------------------------------------------------------------------------------------------------

#[inline]
pub fn delimited_char<'a, S: 'a>() -> impl Parse<char, S> + Combine<char> + 'a
    where
        S: Stream<Item=char>,
{
    char('\'')
        .and_right(escaped().or(not_char('\''))
        )
        .and_left(char('\''))
}
