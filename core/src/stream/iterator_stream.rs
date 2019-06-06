/*
   Copyright 2019 Didier Plaindoux

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

#![allow(dead_code)]

use std::iter::Iterator;
use std::marker::PhantomData;

use crate::stream::stream::Len;
use crate::stream::stream::Stream;

#[derive(Clone)]
pub struct IteratorStream<E, I>(I, usize, PhantomData<E>)
where
    I: Iterator<Item = E>;

impl<'a, E, I> IteratorStream<E, I>
where
    I: Iterator<Item = E>,
{
    pub fn new(s: I) -> IteratorStream<E, I> {
        IteratorStream(s, 0, PhantomData)
    }
}

impl<E, I> Stream for IteratorStream<E, I>
where
    I: Iterator<Item = E> + Clone,
    E: Clone,
{
    type Item = E;

    fn position(&self) -> usize {
        self.1
    }

    fn next(&self) -> (Option<Self::Item>, Self) {
        let mut this = self.clone(); // Ugly code preventing mutability / Check efficiency
        let option = this.0.next();
        let is_some = option.is_some();

        (
            option,
            IteratorStream(this.0, this.1 + (if is_some { 1 } else { 0 }), PhantomData),
        )
    }
}

impl<E, I> Len for IteratorStream<E, I>
where
    I: Iterator<Item = E> + Clone,
{
    fn len(&self) -> usize {
        self.0.clone().count()
    }
}
