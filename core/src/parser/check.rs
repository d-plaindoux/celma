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
use crate::stream::position::Position;

#[derive(Copy, Clone)]
pub struct Check<L, A>(L, PhantomData<A>)
where
    L: Combine<A>;

impl<L, A, B> Combine<Vec<A>> for Check<L, B> where L: Combine<B> {}

impl<L, A, B, S> Parse<Vec<A>, S> for Check<L, B>
where
    L: Parse<B, S> + Combine<B>,
    S: Stream<Item=A>,
{
    fn parse(&self, s: S) -> Response<Vec<A>, S> {
        let Self(p, _) = self;
        let ns = s.clone();
        let start = s.position().offset();
        match p.check(s) {
            Success(_, s, c) => {
                let end = s.position().offset();
                let mut v = Vec::new();
                let mut ns = ns;
                for _ in start..end {
                    let (c,nss) = ns.next();
                    ns = nss;
                    v.push(c.unwrap());
                }
                Success(v, s, c)
            },
            Reject(s, c) => Reject(s, c),
        }
    }

    fn check(&self, s: S) -> Response<(), S> {
        let Self(p, _) = self;
        match p.check(s) {
            Success(v, s, c) => Success(v, s, c),
            Reject(s, _) => Reject(s, false),
        }
    }
}

pub fn check<P, A, B, S>(p: P) -> impl Parse<Vec<A>, S> + Combine<Vec<A>> + Clone
where
    B: Clone,
    S: Stream<Item=A>,
    P: Parse<B, S> + Combine<B> + Clone,
{
    Check(p, PhantomData)
}
