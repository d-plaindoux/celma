#![allow(dead_code)]

use std::iter::Iterator;
use std::marker::PhantomData;

use crate::stream::stream::Len;
use crate::stream::stream::Stream;

#[derive(Clone)]
pub struct IteratorStream<E, I>(I, usize, PhantomData<E>)
    where I: Iterator<Item=E>;

impl<'a, E, I> IteratorStream<E, I>
    where I: Iterator<Item=E>
{
    pub fn new(s: I) -> IteratorStream<E, I> {
        IteratorStream(s, 0, PhantomData)
    }
}

impl<E, I> Stream for IteratorStream<E, I>
    where I: Iterator<Item=E> + Clone,
          E: Clone
{
    type Item = E;

    fn position(&self) -> usize {
        self.1
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        let mut this = self.clone(); // Ugly code preventing mutability / Check efficiency
        let option = this.0.next();
        let is_some = option.is_some();

        (option, IteratorStream(this.0, this.1 + (if is_some { 1 } else { 0 }), PhantomData))
    }
}

impl<E, I> Len for IteratorStream<E, I>
    where I: Iterator<Item=E> + Clone
{
    fn len(&self) -> usize {
        self.0.clone().count()
    }
}
