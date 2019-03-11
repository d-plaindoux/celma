#![allow(dead_code)]

use crate::stream::stream::Stream;

#[derive(Debug)]
pub enum Response<A, S>
where
    S: Stream,
{
    Success(A, S, bool),
    Reject(bool),
}

impl<A, S> Response<A, S>
where
    S: Stream,
{
    pub fn fold<FS, FR, B>(self, success: FS, reject: FR) -> B
    where
        FS: Fn(A, S, bool) -> B,
        FR: Fn(bool) -> B,
    {
        match self {
            Response::Success(a, s, b) => success(a, s, b),
            Response::Reject(b) => reject(b),
        }
    }
}
