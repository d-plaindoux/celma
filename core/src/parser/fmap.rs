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
pub struct FMap<P, A, F, B>(P, F, PhantomData<A>, PhantomData<B>)
where
    P: Combine<A>,
    F: Fn(A) -> B;

impl<P, A, F, B> Combine<B> for FMap<P, A, F, B>
where
    P: Combine<A>,
    F: Fn(A) -> B,
{
}

impl<P, A, F, B, S> Parse<B, S> for FMap<P, A, F, B>
where
    P: Parse<A, S> + Combine<A>,
    F: Fn(A) -> B,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<B, S> {
        let Self(p, f, _, _) = self;

        match p.parse(s) {
            Success(a, s, c) => Success(f(a), s, c),
            Reject(s, c) => Reject(s, c),
        }
    }

    fn check(&self, s: S) -> Response<(), S> {
        let Self(p, _, _, _) = self;

        match p.check(s) {
            Success(_, s, c) => Success((), s, c),
            Reject(s, c) => Reject(s, c),
        }
    }
}

pub trait FMapOperation<P, A, F, B>
where
    P: Combine<A>,
    F: Fn(A) -> B,
{
    fn fmap(self, f: F) -> FMap<P, A, F, B>;
}

impl<P, A, F, B> FMapOperation<P, A, F, B> for P
where
    P: Combine<A>,
    F: Fn(A) -> B,
{
    fn fmap(self, f: F) -> FMap<P, A, F, B> {
        FMap(self, f, PhantomData, PhantomData)
    }
}
