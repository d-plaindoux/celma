#![allow(dead_code)]

use crate::stream::stream::Len;
use crate::stream::stream::Stream;

#[derive(Clone)]
pub struct CharStream<'a>(&'a str, usize);

impl<'a> CharStream<'a> {
    pub fn new(s: &'a str) -> CharStream<'a> {
        CharStream(s, 0)
    }
}

impl<'a> Stream for CharStream<'a> {
    type Item = char;

    fn position(&self) -> usize {
        self.1
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        let option = self.0.chars().next();

        if option.is_some() {
            (
                option,
                CharStream(self.0.get(1..self.0.len()).unwrap_or(""), self.1 + 1),
            )
        } else {
            (None, CharStream(self.0, self.1))
        }
    }
}

impl<'a> Len for CharStream<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
