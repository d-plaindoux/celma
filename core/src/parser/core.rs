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
use std::rc::Rc;

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::parser::satisfy::Satisfy;
use crate::stream::stream::Stream;

// -------------------------------------------------------------------------------------------------

pub fn any<I, S>() -> impl Parse<I, S> + Combine<I>
where
    S: Stream<Item = I>,
    I: Clone,
{
    Satisfy::new((), |_, _| true)
}

// -------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Returns<A>(A)
where
    A: Clone;

impl<A> Combine<A> for Returns<A> where A: Clone {}

impl<A, S> Parse<A, S> for Returns<A>
where
    A: Clone,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<A, S> {
        let Self(v) = self;

        Success(v.clone(), s, false)
    }
}

pub fn returns<A>(v: A) -> Returns<A>
where
    A: Clone,
{
    Returns(v)
}

// -------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Fail<A>(bool, PhantomData<A>);

impl<A> Combine<A> for Fail<A> {}

impl<A, S> Parse<A, S> for Fail<A>
where
    S: Stream,
{
    fn parse(&self, s: S) -> Response<A, S> {
        Reject(s, self.0)
    }
}

pub fn fail<A>(consumed: bool) -> Fail<A> {
    Fail(consumed, PhantomData)
}

// -------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Eos;

impl Combine<()> for Eos {}

impl<S> Parse<(), S> for Eos
where
    S: Stream,
{
    fn parse(&self, s: S) -> Response<(), S> {
        match s.next().0 {
            Some(_) => Reject(s, false),
            None => Success((), s, false),
        }
    }
}

pub fn eos() -> Eos {
    Eos
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct Parser<'a, A, S>(Rc<dyn Parse<A, S> + 'a>)
where
    S: Stream;

impl<'a, A, S> Combine<A> for Parser<'a, A, S> where S: Stream {}

impl<'a, A, S> Parse<A, S> for Parser<'a, A, S>
where
    S: Stream,
{
    fn parse(&self, s: S) -> Response<A, S> {
        let Self(p) = self;

        p.parse(s)
    }

    fn check(&self, s: S) -> Response<(), S> {
        let Self(p) = self;

        p.check(s)
    }
}

pub fn parser<'a, P, A, S>(p: P) -> Parser<'a, A, S>
where
    P: Parse<A, S> + 'a,
    S: Stream,
{
    Parser(Rc::new(p))
}

// -------------------------------------------------------------------------------------------------
