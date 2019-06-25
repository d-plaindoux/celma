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
pub struct Bind<P, A, F, R, B>(P, F, PhantomData<A>, PhantomData<B>)
where
    P: Combine<A>,
    R: Combine<B>,
    F: Fn(A) -> R;

impl<P, A, F, R, B> Combine<B> for Bind<P, A, F, R, B>
where
    P: Combine<A>,
    R: Combine<B>,
    F: Fn(A) -> R,
{
}

impl<P, A, F, R, B, S> Parse<B, S> for Bind<P, A, F, R, B>
where
    P: Parse<A, S> + Combine<A>,
    R: Parse<B, S> + Combine<B>,
    F: Fn(A) -> R,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<B, S> {
        let Self(p, f, _, _) = self;

        match p.parse(s) {
            Success(a, sa, ca) => match f(a).parse(sa) {
                Success(b, sb, cb) => Success(b, sb, ca || cb),
                Reject(sb, c) => Reject(sb, c),
            },
            Reject(sa, c) => Reject(sa, c),
        }
    }

    fn check(&self, s: S) -> Response<(), S> {
        let Self(p, f, _, _) = self;

        match p.parse(s) {
            Success(a, sa, ca) => match f(a).check(sa) {
                Success(_, sb, cb) => Success((), sb, ca || cb),
                Reject(sb, c) => Reject(sb, c),
            },
            Reject(sa, c) => Reject(sa, c),
        }
    }
}

pub trait BindOperation<P, A, F, R, B>
where
    P: Combine<A>,
    R: Combine<B>,
    F: Fn(A) -> R,
{
    fn bind(self, f: F) -> Bind<P, A, F, R, B>;
}

impl<P, A, F, R, B> BindOperation<P, A, F, R, B> for P
where
    P: Combine<A>,
    R: Combine<B>,
    F: Fn(A) -> R,
{
    fn bind(self, f: F) -> Bind<P, A, F, R, B> {
        Bind(self, f, PhantomData, PhantomData)
    }
}

impl<P, A, F, R, B, S> First<S> for Bind<P, A, F, R, B>
    where
        P: Parse<A, S> + Combine<A>,
        R: Parse<B, S> + Combine<B>,
        F: Fn(A) -> R,
        S: Stream,
{
    fn first(&self) -> Vec<Token<S::Item>> {
        unimplemented!()
    }
}
