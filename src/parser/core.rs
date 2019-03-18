use std::marker::PhantomData;

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::parser::satisfy::Satisfy;
use crate::stream::stream::Stream;
use std::rc::Rc;

// -------------------------------------------------------------------------------------------------

pub fn any<I>() -> Satisfy<impl Fn(&I) -> bool, I> {
    Satisfy::new(|_| true)
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Fail<A>(bool, PhantomData<A>);

impl<A> Combine<A> for Fail<A> {}

impl<A, S> Parse<A, S> for Fail<A>
where
    S: Stream,
{
    fn parse(&self, _s: S) -> Response<A, S> {
        Reject(self.0)
    }
}

pub fn fail<A>(consumed: bool) -> Fail<A> {
    Fail(consumed, PhantomData)
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct Eos;

impl Combine<()> for Eos {}

impl<S> Parse<(), S> for Eos
where
    S: Stream,
{
    fn parse(&self, s: S) -> Response<(), S> {
        match s.next().0 {
            Some(_) => Reject(false),
            None => Success((), s, false),
        }
    }
}

pub fn eos() -> Eos {
    Eos
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct Parser<A, S>(Rc<dyn Parse<A, S>>)
where
    S: Stream;

impl<A, S> Combine<A> for Parser<A, S> where S: Stream {}

impl<A, S> Parse<A, S> for Parser<A, S>
where
    S: Stream,
{
    fn parse(&self, s: S) -> Response<A, S> {
        let Self(p) = self;

        p.parse(s)
    }
}

pub fn parser<P: 'static, A, S>(p: P) -> Parser<A, S>
where
    P: Parse<A, S>,
    S: Stream,
{
    Parser(Rc::new(p))
}
