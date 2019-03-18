use std::marker::PhantomData;

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;

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
                    Reject(false)
                }
            }
            _ => Reject(false),
        }
    }
}
