use std::marker::PhantomData;

use crate::parser::fmap::FMap;
use crate::parser::fmap::FMapOperation;
use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;

#[derive(Copy, Clone)]
pub struct And<L, R, A, B>(L, R, PhantomData<A>, PhantomData<B>)
where
    L: Combine<A>,
    R: Combine<B>;

impl<L, R, A, B> Combine<(A, B)> for And<L, R, A, B>
where
    L: Combine<A>,
    R: Combine<B>,
{
}

impl<L, R, A, B, S> Parse<(A, B), S> for And<L, R, A, B>
where
    L: Parse<A, S> + Combine<A>,
    R: Parse<B, S> + Combine<B>,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<(A, B), S> {
        let Self(l, r, _, _) = self;

        match l.parse(s) {
            Success(a, s, ba) => match r.parse(s) {
                Success(b, s, bb) => Success((a, b), s, ba || bb),
                Reject(s, bb) => Reject(s, ba || bb),
            },
            Reject(s, ba) => Reject(s, ba),
        }
    }
}

pub trait AndOperation<L, R, A, B>
where
    L: Combine<A>,
    R: Combine<B>,
{
    fn and(self, a: R) -> And<L, R, A, B>;
    fn and_left<'a>(self, a: R) -> LeftProjection<'a, L, R, A, B>;
    fn and_right<'a>(self, a: R) -> RightProjection<'a, L, R, A, B>;
}

impl<L, R, A, B> AndOperation<L, R, A, B> for L
where
    L: Combine<A>,
    R: Combine<B>,
{
    #[inline]
    fn and(self, a: R) -> And<L, R, A, B> {
        And(self, a, PhantomData, PhantomData)
    }

    fn and_left<'a>(self, a: R) -> LeftProjection<'a, L, R, A, B> {
        self.and(a).fmap(&|(l, _)| l)
    }

    fn and_right<'a>(self, a: R) -> RightProjection<'a, L, R, A, B> {
        self.and(a).fmap(&|(_, r)| r)
    }
}

type LeftProjection<'a, L, R, A, B> = FMap<And<L, R, A, B>, (A, B), &'a dyn Fn((A, B)) -> A, A>;
type RightProjection<'a, L, R, A, B> = FMap<And<L, R, A, B>, (A, B), &'a dyn Fn((A, B)) -> B, B>;

pub trait AndProjection<L, R, A, B>
where
    L: Combine<A>,
    R: Combine<B>,
{
    fn left<'a>(self) -> LeftProjection<'a, L, R, A, B>;
    fn right<'a>(self) -> RightProjection<'a, L, R, A, B>;
}

impl<L, R, A, B> AndProjection<L, R, A, B> for And<L, R, A, B>
where
    L: Combine<A>,
    R: Combine<B>,
{
    fn left<'a>(self) -> LeftProjection<'a, L, R, A, B> {
        self.fmap(&|(l, _)| l)
    }

    fn right<'a>(self) -> RightProjection<'a, L, R, A, B> {
        self.fmap(&|(_, r)| r)
    }
}
