use std::marker::PhantomData;

use crate::parser::parser::{Combine, Parse};
use crate::parser::response::Response;
use crate::stream::stream::Stream;

#[derive(Clone)]
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
    S: Stream,
    P: Parse<A, S> + Combine<A>,
    F: Fn() -> P,
{
    fn parse(&self, s: S) -> Response<A, S> {
        let Self(f, _, _) = self;

        f().parse(s)
    }
}

pub fn lazy<F, P, A, S>(f: F) -> impl Parse<A, S> + Combine<A>
where
    S: Stream,
    P: Parse<A, S> + Combine<A>,
    F: Fn() -> P + Clone,
{
    Lazy(f, PhantomData, PhantomData)
}
