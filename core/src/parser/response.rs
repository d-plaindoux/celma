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

use crate::stream::specs::Stream;

#[derive(Debug)]
pub enum Response<A, S>
where
    S: Stream,
{
    Success(A, S, bool),
    Reject(S, bool),
}

impl<A, S> Response<A, S>
where
    S: Stream,
{
    pub fn fold<FS, FR, B>(self, success: FS, reject: FR) -> B
    where
        FS: Fn(A, S, bool) -> B,
        FR: Fn(S, bool) -> B,
    {
        match self {
            Response::Success(a, s, b) => success(a, s, b),
            Response::Reject(s, b) => reject(s, b),
        }
    }
    pub fn map<F, B>(self, f: F) -> Response<B, S>
    where
        F: Fn(A) -> B,
    {
        self.fold(
            |a, s, b| Response::Success(f(a), s, b),
            |s, b| Response::Reject(s, b),
        )
    }
}
