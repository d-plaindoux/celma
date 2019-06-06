use std::marker::PhantomData;

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;

#[derive(Copy, Clone)]
pub struct Optional<L, A>(L, PhantomData<A>)
where
    L: Combine<A>;

impl<L, A> Combine<Option<A>> for Optional<L, A> where L: Combine<A> {}

impl<L, A, S> Parse<Option<A>, S> for Optional<L, A>
where
    L: Parse<A, S> + Combine<A>,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<Option<A>, S> {
        let Self(p, _) = self;

        match p.parse(s.clone()) {
            Success(v, s, c) => Success(Some(v), s, c),
            Reject(ns, c) => {
                if c {
                    Reject(ns, c)
                } else {
                    Success(None, s, false)
                }
            }
        }
    }
}

pub trait OptionalOperation<L, A>
where
    L: Combine<A>,
{
    fn opt(self) -> Optional<L, A>;
}

impl<L, A> OptionalOperation<L, A> for L
where
    L: Combine<A>,
{
    fn opt(self) -> Optional<L, A> {
        Optional(self, PhantomData)
    }
}
