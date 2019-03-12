#![allow(dead_code)]

use crate::stream::stream::Len;
use crate::stream::stream::Stream;

#[derive(Clone)]
pub struct VecStream<'a, A>(&'a [A], usize);

impl<'a, A> VecStream<'a, A> {
    pub fn new(v: &'a [A]) -> VecStream<'a, A> {
        VecStream(v, 0)
    }
}

impl<'a, A> Stream for VecStream<'a, A>
where
    A: Clone,
{
    type Item = A;

    fn position(&self) -> usize {
        self.1
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        let option = self.0.get(self.1);

        if option.is_some() {
            (option.cloned(), VecStream(self.0, self.1 + 1))
        } else {
            (option.cloned(), VecStream(self.0, self.1))
        }
    }
}

impl<'a, A> Len for VecStream<'a, A> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
