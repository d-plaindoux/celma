#![allow(dead_code)]

use crate::stream::stream::Len;
use crate::stream::stream::Stream;

#[derive(Clone)]
pub struct U8Stream<'a>(&'a [u8], usize);

impl<'a> U8Stream<'a> {
    pub fn new(s: &'a [u8]) -> U8Stream<'a> {
        U8Stream(s, 0)
    }
}

impl<'a> Stream for U8Stream<'a> {
    type Item = u8;

    fn position(&self) -> usize {
        self.1
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        if self.1 < self.0.len() {
            (Some(self.0[self.1]), U8Stream(self.0, self.1 + 1))
        } else {
            (None, U8Stream(self.0, self.1))
        }
    }
}

impl<'a> Len for U8Stream<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
