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

pub trait Position: Default {
    fn step(&self, newline: bool) -> Self;

    fn offset(&self) -> usize;

    fn char_number(&self) -> usize {
        self.offset()
    }

    fn line_number(&self) -> usize {
        0
    }
}

/// Basic position implemented with an index on the char
///
/// Does not count lines
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CharIndex(usize);

impl Position for CharIndex {
    #[inline]
    fn step(&self, _: bool) -> Self {
        Self(self.0 + 1)
    }

    #[inline]
    fn offset(&self) -> usize {
        self.0
    }
}

impl Display for CharIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<usize> for CharIndex {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

/// Char based position with lines & columns
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharPosition {
    /// Char index
    pub char_index: usize,

    /// 1-based line
    pub line: usize,

    /// 0-based column
    pub column: usize,
}

impl Default for CharPosition {
    fn default() -> Self {
        Self {
            char_index: 0,
            line: 1,
            column: 0,
        }
    }
}

impl Display for CharPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Position for CharPosition {
    #[inline]
    fn step(&self, newline: bool) -> Self {
        if newline {
            Self {
                char_index: self.char_index + 1,
                line: self.line + 1,
                column: 0,
            }
        } else {
            Self {
                char_index: self.char_index + 1,
                line: self.line,
                column: self.column + 1,
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
