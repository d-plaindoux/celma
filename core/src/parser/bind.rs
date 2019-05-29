use std::marker::PhantomData;

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;

#[derive(Copy, Clone)]
pub struct Bind<P, A, F, R, B>(P, F, PhantomData<A>, PhantomData<B>)
where
    P: Combine<A>,
    R: Combine<B>,
    F: Fn(A) -> R;

impl<P, A, F, R, B> Combine<B> for Bind<P, A, F, R, B>
where
    P: Combine<A>,
    R: Combine<B>,
    F: Fn(A) -> R,
{
}

impl<P, A, F, R, B, S> Parse<B, S> for Bind<P, A, F, R, B>
where
    P: Parse<A, S> + Combine<A>,
    R: Parse<B, S> + Combine<B>,
    F: Fn(A) -> R,
    S: Stream,
{
    fn parse(&self, s: S) -> Response<B, S> {
        let Self(p, f, _, _) = self;

        match p.parse(s) {
            Success(a, sa, ca) => match f(a).parse(sa) {
                Success(b, sb, cb) => Success(b, sb, ca || cb),
                Reject(c) => Reject(c),
            },
            Reject(c) => Reject(c),
        }
    }
}

pub trait BindOperation<P, A, F, R, B>
where
    P: Combine<A>,
    R: Combine<B>,
    F: Fn(A) -> R,
{
    fn bind(self, f: F) -> Bind<P, A, F, R, B>;
}

impl<P, A, F, R, B> BindOperation<P, A, F, R, B> for P
where
    P: Combine<A>,
    R: Combine<B>,
    F: Fn(A) -> R,
{
    fn bind(self, f: F) -> Bind<P, A, F, R, B> {
        Bind(self, f, PhantomData, PhantomData)
    }
}
