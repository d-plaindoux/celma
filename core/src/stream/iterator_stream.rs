#![allow(dead_code)]

use std::iter::Iterator;

use crate::stream::stream::Len;
use crate::stream::stream::Stream;

#[derive(Clone)]
pub struct IteratorStream<E>(Iterator<Item=E>, usize);

impl<'a> IteratorStream<E> {
    pub fn new(s: Iterator<Item=E>) -> IteratorStream<E> {
        IteratorStream(s, 0)
    }
}

impl<E> Stream for IteratorStream<E> {
    type Item = char;

    fn position(&self) -> usize {
        self.1
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        let option = self.0.next();

        (option, IteratorStream(self.0, self.1 + (if option.is_some() { 1 } else { 0 }))
    }
}

impl<'a> Len for CharStream<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
