#![allow(dead_code)]

use std::marker::PhantomData;

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;

pub struct Or<L, R, A>(L, R, PhantomData<A>)
where
    L: Combine<A>,
    R: Combine<A>;

impl<L, R, A> Combine<A> for Or<L, R, A>
where
    L: Combine<A>,
    R: Combine<A>,
{
}

impl<L, R, A, S> Parse<A, S> for Or<L, R, A>
where
    L: Parse<A, S> + Combine<A>,
    R: Parse<A, S> + Combine<A>,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<A, S> {
        let Self(l, r, _) = self;

        match l.parse(s.clone()) {
            Success(a, na, ba) => Success(a, na, ba),
            Reject(ba) => {
                if ba {
                    Reject(ba)
                } else {
                    r.parse(s)
                }
            }
        }
    }
}

pub trait OrOperation<L, R, A>
where
    L: Combine<A>,
    R: Combine<A>,
{
    fn or(self, a: R) -> Or<L, R, A>;
}

impl<L, R, A> OrOperation<L, R, A> for L
where
    L: Combine<A>,
    R: Combine<A>,
{
    #[inline]
    fn or(self, a: R) -> Or<L, R, A> {
        Or(self, a, PhantomData)
    }
}
