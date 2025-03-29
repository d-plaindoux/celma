/*
   Copyright 2019-2025 Didier Plaindoux

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

use std::fmt::Display;
use std::marker::PhantomData;

use super::end_line::EndLine;

pub trait Position: Default {
    type Item;

    fn step(&self, a: &Self::Item) -> Self;

    fn offset(&self) -> usize;

    fn char_number(&self) -> usize {
        self.offset()
    }

    fn line_number(&self) -> usize {
        0
    }
}

/// Basic position implemented with an index
///
/// Does not count lines
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IndexPosition<A>(usize, PhantomData<A>);

impl<A> Position for IndexPosition<A> {
    type Item = A;

    #[inline]
    fn step(&self, _: &A) -> Self {
        Self(self.0 + 1, PhantomData)
    }

    #[inline]
    fn offset(&self) -> usize {
        self.0
    }
}

impl<A> Default for IndexPosition<A> {
    fn default() -> Self {
        Self(0, PhantomData)
    }
}

impl<A> Display for IndexPosition<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<A> PartialEq<usize> for IndexPosition<A> {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

/// Line and column based position
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineColumnPosition<A> {
    /// Char index
    pub char_index: usize,

    /// 1-based line
    pub line: usize,

    /// 0-based column
    pub column: usize,

    marker: PhantomData<A>,
}

impl<A> LineColumnPosition<A> {
    #[doc(hidden)]
    pub fn new(char_index: usize, line: usize, column: usize) -> Self {
        Self {
            char_index,
            line,
            column,
            marker: PhantomData,
        }
    }
}

impl<A> Position for LineColumnPosition<A>
where
    A: EndLine,
{
    type Item = A;

    #[inline]
    fn step(&self, a: &A) -> Self {
        if a.is_end_line() {
            Self {
                char_index: self.char_index + 1,
                line: self.line + 1,
                column: 0,
                marker: PhantomData,
            }
        } else {
            Self {
                char_index: self.char_index + 1,
                line: self.line,
                column: self.column + 1,
                marker: PhantomData,
            }
        }
    }

    fn offset(&self) -> usize {
        self.column
    }

    fn char_number(&self) -> usize {
        self.char_index
    }

    fn line_number(&self) -> usize {
        self.line
    }
}

impl<A> Default for LineColumnPosition<A> {
    fn default() -> Self {
        Self {
            char_index: 0,
            line: 1,
            column: 0,
            marker: PhantomData,
        }
    }
}

impl<A> Display for LineColumnPosition<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
