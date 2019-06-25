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

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;
use crate::parser::ff::{First, Token};

#[derive(Copy, Clone)]
pub struct Optional<L, A>(L, PhantomData<A>)
where
    L: Combine<A>;

impl<L, A> Combine<Option<A>> for Optional<L, A> where L: Combine<A> {}

impl<L, A, S> Parse<Option<A>, S> for Optional<L, A>
where
    L: Parse<A, S> + Combine<A>,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<Option<A>, S> {
        let Self(p, _) = self;

        match p.parse(s.clone()) {
            Success(v, s, c) => Success(Some(v), s, c),
            Reject(ns, c) => {
                if c {
                    Reject(ns, c)
                } else {
                    Success(None, s, false)
                }
            }
        }
    }

    fn check(&self, s: S) -> Response<(), S> {
        let Self(p, _) = self;

        match p.check(s.clone()) {
            Success(v, s, c) => Success(v, s, c),
            Reject(ns, c) => {
                if c {
                    Reject(ns, c)
                } else {
                    Success((), s, false)
                }
            }
        }
    }
}

pub trait OptionalOperation<L, A>
where
    L: Combine<A>,
{
    fn opt(self) -> Optional<L, A>;
}

impl<L, A> OptionalOperation<L, A> for L
where
    L: Combine<A>,
{
    fn opt(self) -> Optional<L, A> {
        Optional(self, PhantomData)
    }
}

impl<L, A, S> First<S> for Optional<L, A>
    where
        L: First<S> + Parse<A, S> + Combine<A>,
        S: Stream,
{
    fn first(&self) -> Vec<Token<S::Item>> {
        let Self(p, _) = self;
        let mut o = vec![Token::NoAtom];

        o.append(&mut p.first());

        o
    }
}
