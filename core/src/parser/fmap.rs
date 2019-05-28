use std::marker::PhantomData;

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;

#[derive(Clone)]
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
            Reject(c) => Reject(c),
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
