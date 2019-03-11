#![allow(dead_code)]

use crate::parser::response::Response;
use crate::stream::stream::Stream;

pub trait Combine<A> {}

pub trait Parse<A, S>
where
    S: Stream,
{
    fn parse(&self, s: S) -> Response<A, S>;
}
