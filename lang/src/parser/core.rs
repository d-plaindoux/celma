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

use celma_core::parser::core::{Fail, Parser, Returns};
use celma_core::stream::stream::Stream;

use crate::parser::ff::{First, Token};

impl<A, S> First<S> for Returns<A>
where
    A: Clone,
    S: Stream,
{
    fn first(&self) -> Vec<Token<S::Item>> {
        vec![Token::NoAtom]
    }
}

// -------------------------------------------------------------------------------------------------

impl<A, S> First<S> for Fail<A>
where
    S: Stream,
{
    fn first(&self) -> Vec<Token<S::Item>> {
        vec![Token::NoAtom]
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Eos;

impl<S> First<S> for Eos
where
    S: Stream,
{
    fn first(&self) -> Vec<Token<S::Item>> {
        vec![Token::NoAtom]
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, A, S> First<S> for Parser<'a, A, S>
where
    S: Stream,
{
    fn first(&self) -> Vec<Token<S::Item>> {
        vec![Token::NoAtom] // TODO(didier) how can the internal parser (opaque) can be used !?
    }
}

// -------------------------------------------------------------------------------------------------
