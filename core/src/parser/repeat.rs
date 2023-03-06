/*
   Copyright 2019-2023 Didier Plaindoux

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
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;

#[derive(Copy, Clone)]
pub struct Repeat<L, A>(bool, L, PhantomData<A>)
where
    L: Combine<A>;

impl<L, A> Combine<Vec<A>> for Repeat<L, A> where L: Combine<A> {}

impl<L, A, S> Parse<Vec<A>, S> for Repeat<L, A>
where
    L: Parse<A, S> + Combine<A>,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<Vec<A>, S> {
        let Self(can_be_empty, p, _) = self;

        let mut values = Vec::new();
        let mut consumed = false;
        let mut source = s;

        loop {
            match p.parse(source.clone()) {
                Success(v, s, c) => {
                    values.push(v);
                    consumed = c || consumed;
                    source = s;
                }
                Reject(s, c) => {
                    if c {
                        return Reject(s, c);
                    }

                    if !*can_be_empty && values.is_empty() {
                        return Reject(s, consumed);
                    }

                    return Success(values, source, consumed);
                }
            }
        }
    }

    fn check(&self, s: S) -> Response<(), S> {
        let Self(can_be_empty, p, _) = self;

        let mut empty = true;
        let mut consumed = false;
        let mut source = s;

        loop {
            match p.check(source.clone()) {
                Success(_, s, c) => {
                    empty = false;
                    consumed = c || consumed;
                    source = s;
                }
                Reject(s, c) => {
                    if c {
                        return Reject(s, c);
                    }

                    if !*can_be_empty && empty {
                        return Reject(s, consumed);
                    }

                    return Success((), source, consumed);
                }
            }
        }
    }
}

pub trait RepeatOperation<L, A>
where
    L: Combine<A>,
{
    fn rep(self) -> Repeat<L, A>;
    fn opt_rep(self) -> Repeat<L, A>;
}

impl<L, A> RepeatOperation<L, A> for L
where
    L: Combine<A>,
{
    fn rep(self) -> Repeat<L, A> {
        Repeat(false, self, PhantomData)
    }

    fn opt_rep(self) -> Repeat<L, A> {
        Repeat(true, self, PhantomData)
    }
}
