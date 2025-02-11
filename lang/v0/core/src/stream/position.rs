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

pub trait Position {
    fn new() -> Self;

    fn step(&self, newline: bool) -> Self;

    fn offset(&self) -> usize;

    fn char_number(&self) -> usize {
        self.offset()
    }

    fn line_number(&self) -> usize {
        0
    }
}

impl Position for usize {
    fn new() -> Self {
        0
    }

    #[inline]
    fn step(&self, _: bool) -> Self {
        self + 1
    }

    fn offset(&self) -> usize {
        *self
    }
}

impl Position for (usize, usize, usize) {
    fn new() -> Self {
        (0, 1, 0)
    }

    #[inline]
    fn step(&self, newline: bool) -> Self {
        if newline {
            (self.0 + 1, self.1 + 1, 0)
        } else {
            (self.0 + 1, self.1, self.2 + 1)
        }
    }

    fn offset(&self) -> usize {
        self.0
    }

    fn char_number(&self) -> usize {
        self.1
    }

    fn line_number(&self) -> usize {
        self.2
    }
}
