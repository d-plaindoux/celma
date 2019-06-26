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

use std::marker::PhantomData;

use crate::parser::ff::{First, Token};
use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;

#[derive(Copy, Clone)]
pub struct Not<L, A>(L, PhantomData<A>)
where
    L: Combine<A>;

impl<L, A> Combine<A> for Not<L, A> where L: Combine<A> {}

impl<L, A, S> Parse<A, S> for Not<L, A>
where
    L: Parse<A, S> + Combine<A>,
    S: Stream<Item = A>,
{
    fn parse(&self, s: S) -> Response<A, S> {
        let Self(p, _) = self;

        match p.parse(s.clone()) {
            Success(_, s, _) => Reject(s, false),
            _ => match s.next() {
                (Some(v), s) => Success(v, s, true),
                _ => Reject(s, false),
            },
        }
    }

    fn check(&self, s: S) -> Response<(), S> {
        let Self(p, _) = self;

        match p.check(s.clone()) {
            Success(_, s, _) => Reject(s, false),
            _ => match s.next() {
                (Some(_), s) => Success((), s, true),
                _ => Reject(s, false),
            },
        }
    }
}

pub trait NotOperation<L, A>
where
    L: Combine<A>,
{
    fn not(self) -> Not<L, A>;
}

impl<L, A> NotOperation<L, A> for L
where
    L: Combine<A>,
{
    fn not(self) -> Not<L, A> {
        Not(self, PhantomData)
    }
}

impl<L, A, S> First<S> for Not<L, A>
where
    L: First<S> + Parse<A, S> + Combine<A>,
    S: Stream<Item = A>,
    A: Clone,
{
    fn first(&self) -> Vec<Token<S::Item>> {
        let Self(p, _) = self;

        p.first().iter().map(|t| t.negate()).collect::<Vec<_>>()
    }
}
