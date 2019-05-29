use std::marker::PhantomData;

use crate::parser::parser::{Combine, Parse};
use crate::parser::response::Response;
use crate::stream::stream::Stream;

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
}

pub fn lazy<F, P, A, S>(f: F) -> impl Parse<A, S> + Combine<A> + Clone
where
    A: Clone,
    S: Stream,
    P: Parse<A, S> + Combine<A> + Clone,
    F: Fn() -> P + Clone,
{
    Lazy(f, PhantomData, PhantomData)
}
