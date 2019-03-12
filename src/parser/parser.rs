#![allow(dead_code)]

use crate::parser::response::Response;
use crate::stream::stream::Stream;

pub trait Combine<A> {}

pub trait Parse<A, O>
where
    O: Stream,
{
    fn parse(&self, s: O) -> Response<A, O>;
}
