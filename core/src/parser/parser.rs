/*
   Copyright 2019-2020 Didier Plaindoux

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

use crate::parser::response::Response;
use crate::parser::response::Response::{Reject, Success};
use crate::stream::stream::Stream;

pub trait Combine<A> {}

pub trait Parse<A, S>
where
    S: Stream,
{
    fn parse(&self, s: S) -> Response<A, S>;

    fn check(&self, s: S) -> Response<(), S> {
        match self.parse(s) {
            Success(_, s, c) => Success((), s, c),
            Reject(s, c) => Reject(s, c),
        }
    }
}
