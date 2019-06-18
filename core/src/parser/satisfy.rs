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
pub struct Satisfy<E, I>(E, PhantomData<I>)
where
    E: Fn(&I) -> bool;

impl<E, I> Satisfy<E, I>
where
    E: Fn(&I) -> bool,
{
    pub fn new(e: E) -> Satisfy<E, I>
    where
        E: Fn(&I) -> bool,
    {
        Satisfy(e, PhantomData)
    }
}

impl<E, I> Combine<I> for Satisfy<E, I> where E: Fn(&I) -> bool {}

impl<A, I, S> Parse<I, S> for Satisfy<A, I>
where
    A: Fn(&I) -> bool,
    S: Stream<Item = I>,
    I: Clone,
{
    fn parse(&self, s: S) -> Response<I, S> {
        let Self(predicate, _) = self;

        match s.next() {
            (Some(c), p) => {
                if predicate(&c) {
                    Success(c, p, true)
                } else {
                    Reject(p, false)
                }
            }
            (None, p) => Reject(p, false),
        }
    }

    fn check(&self, s: S) -> Response<(), S> {
        let Self(predicate, _) = self;

        match s.next() {
            (Some(c), p) => {
                if predicate(&c) {
                    Success((), p, true)
                } else {
                    Reject(p, false)
                }
            }
            (None, p) => Reject(p, false),
        }
    }
}

#[inline]
pub fn not<S, E>(c: E) -> impl Parse<E, S> + Combine<E>
where
    E: Eq + Copy,
    S: Stream<Item = E>,
{
    Satisfy::new(move |&v| v != c)
}

#[inline]
pub fn eq<S, E>(c: E) -> impl Parse<E, S> + Combine<E>
where
    E: Eq + Copy,
    S: Stream<Item = E>,
{
    Satisfy::new(move |&v| v == c)
}
