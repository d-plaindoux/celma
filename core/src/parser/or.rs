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

use std::marker::PhantomData;

use crate::parser::specs::Combine;
use crate::parser::specs::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::stream::specs::Stream;

#[derive(Copy, Clone)]
pub struct Or<L, R, A>(L, R, PhantomData<A>)
where
    L: Combine<A>,
    R: Combine<A>;

impl<L, R, A> Combine<A> for Or<L, R, A>
where
    L: Combine<A>,
    R: Combine<A>,
{
}

impl<L, R, A, S> Parse<A, S> for Or<L, R, A>
where
    L: Parse<A, S> + Combine<A>,
    R: Parse<A, S> + Combine<A>,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<A, S> {
        let Self(l, r, _) = self;

        match l.parse(s.clone()) {
            Reject(_, false) => r.parse(s),
            r => r,
        }
    }

    fn check(&self, s: S) -> Response<(), S> {
        let Self(l, r, _) = self;

        match l.check(s.clone()) {
            Reject(_, false) => r.check(s),
            r => r,
        }
    }
}

pub trait OrOperation<L, R, A>
where
    L: Combine<A>,
    R: Combine<A>,
{
    fn or(self, a: R) -> Or<L, R, A>;
}

impl<L, R, A> OrOperation<L, R, A> for L
where
    L: Combine<A>,
    R: Combine<A>,
{
    #[inline]
    fn or(self, a: R) -> Or<L, R, A> {
        Or(self, a, PhantomData)
    }
}
