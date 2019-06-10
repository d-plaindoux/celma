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

#[derive(Copy, Clone)]
pub struct ATry<L, A>(L, PhantomData<A>)
where
    L: Combine<A>;

impl<L, A> Combine<A> for ATry<L, A> where L: Combine<A> {}

impl<L, A, S> Parse<A, S> for ATry<L, A>
where
    L: Parse<A, S> + Combine<A>,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<A, S> {
        let Self(p, _) = self;
        match p.parse(s) {
            Success(v, s, c) => Success(v, s, c),
            Reject(s, _) => Reject(s, false),
        }
    }
}

pub fn a_try<P, A, S>(p: P) -> impl Parse<A, S> + Combine<A> + Clone
where
    A: Clone,
    S: Stream,
    P: Parse<A, S> + Combine<A> + Clone,
{
    ATry(p, PhantomData)
}
