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

use crate::parser::specs::{Combine, Parse};
use crate::parser::response::Response;
use crate::stream::specs::Stream;

#[derive(Copy, Clone)]
pub struct Lazy<F, P, A>(F, PhantomData<P>, PhantomData<A>)
where
    P: Combine<A>,
    F: Fn() -> P;

impl<F, P, A> Combine<A> for Lazy<F, P, A>
where
    P: Combine<A>,
    F: Fn() -> P,
{
}

impl<F, P, A, S> Parse<A, S> for Lazy<F, P, A>
where
    P: Parse<A, S> + Combine<A>,
    F: Fn() -> P,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<A, S> {
        let Self(f, _, _) = self;

        f().parse(s)
    }

    fn check(&self, s: S) -> Response<(), S> {
        let Self(f, _, _) = self;

        f().check(s)
    }
}

pub fn lazy<F, P, A, S>(f: F) -> impl Parse<A, S> + Combine<A>
where
    A: Clone,
    S: Stream,
    P: Parse<A, S> + Combine<A>,
    F: Fn() -> P,
{
    Lazy(f, PhantomData, PhantomData)
}
