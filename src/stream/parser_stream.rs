#![allow(dead_code)]

use std::marker::PhantomData;

use crate::parser::parser::Combine;
use crate::parser::parser::Parse;
use crate::parser::response::Response::Reject;
use crate::parser::response::Response::Success;
use crate::stream::stream::Stream;

pub struct ParserStream<'a, P, A, S>(&'a P, S, PhantomData<A>)
where
    P: Combine<A> + Parse<A, S>,
    S: Stream;

impl<'a, P, A, S> ParserStream<'a, P, A, S>
where
    P: Combine<A> + Parse<A, S>,
    S: Stream,
{
    #[inline]
    pub fn new(p: &'a P, s: S) -> Self {
        ParserStream(p, s, PhantomData)
    }
}

impl<'a, P, A, S> Clone for ParserStream<'a, P, A, S>
where
    P: Combine<A> + Parse<A, S>,
    S: Stream,
{
    fn clone(&self) -> Self {
        ParserStream(self.0, self.1.clone(), PhantomData)
    }
}

impl<'a, P, A, S> Stream for ParserStream<'a, P, A, S>
where
    P: Combine<A> + Parse<A, S>,
    S: Stream,
{
    type Item = A;

    fn position(&self) -> usize {
        self.1.position()
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        match self.0.parse(self.1.clone()) {
            Success(a, s, _) => (Some(a), ParserStream::new(self.0, s.clone())),
            Reject(_) => (None, ParserStream::new(self.0, self.1.clone())),
        }
    }
}
