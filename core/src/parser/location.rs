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
use crate::stream::stream::{Position, Stream};

#[derive(Copy, Clone)]
pub struct Location<A> {
    position: Position,
    value: A,
}

#[derive(Copy, Clone)]
pub struct Locate<P, A>(P, PhantomData<A>)
where
    P: Combine<A>;

impl<P, A> Combine<Location<A>> for Locate<P, A> where P: Combine<A> {}

impl<P, A, S> Parse<Location<A>, S> for Locate<P, A>
where
    P: Parse<A, S> + Combine<A>,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<Location<A>, S> {
        let Self(p, _) = self;

        match p.parse(s) {
            Success(a, s, c) => {
                let l = Location {
                    position: s.position(),
                    value: a,
                };

                Success(l, s, c)
            }
            Reject(s, c) => Reject(s, c),
        }
    }
}
