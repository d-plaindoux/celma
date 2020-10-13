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

use std::marker::PhantomData;

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;

#[derive(Copy, Clone)]
pub struct Lookahead<P, A>(P, PhantomData<A>)
where
    P: Combine<A>;

impl<P, A> Combine<A> for Lookahead<P, A> where P: Combine<A> {}

impl<P, A, S> Parse<A, S> for Lookahead<P, A>
where
    P: Parse<A, S> + Combine<A>,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<A, S> {
        let Self(p, _) = self;

        match p.parse(s.clone()) {
            Success(a, _, ba) => Success(a, s, ba),
            Reject(s, ba) => Reject(s, ba),
        }
    }

    fn check(&self, s: S) -> Response<(), S> {
        let Self(p, _) = self;

        match p.check(s.clone()) {
            Success(a, _, ba) => Success(a, s, ba),
            Reject(s, ba) => Reject(s, ba),
        }
    }
}

pub fn lookahead<P, A, S>(p: P) -> impl Parse<A, S> + Combine<A>
where
    A: Clone,
    S: Stream,
    P: Parse<A, S> + Combine<A>,
{
    Lookahead(p, PhantomData)
}
