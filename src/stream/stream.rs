#![allow(dead_code)]

// How this copy can be removed?

pub trait Stream: Clone {
    type Item;

    fn position(&self) -> usize;

    fn next(&self) -> (Option<Self::Item>, Self);
}

pub trait Len {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
