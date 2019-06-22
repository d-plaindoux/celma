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

use crate::parser::ff::{First, Token};
use celma_core::parser::fmap::FMap;
use celma_core::parser::parser::{Combine, Parse};
use celma_core::stream::stream::Stream;

impl<P, A, F, B, S> First<S> for FMap<P, A, F, B>
where
    P: First<S> + Parse<A, S> + Combine<A>,
    F: Fn(A) -> B,
    S: Stream,
{
    fn first(&self) -> Vec<Token<S::Item>> {
        let Self(p, _, _, _) = self;

        p.first()
    }
}