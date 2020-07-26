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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token<E> {
    Atom(E),
    NotAtom(E),
    NoAtom,
    AllAtom,
}

impl<E> Token<E>
where
    E: Clone,
{
    pub fn negate(&self) -> Self {
        match self {
            Token::NoAtom => Token::AllAtom,
            Token::AllAtom => Token::NoAtom,
            Token::NotAtom(v) => Token::Atom(v.clone()),
            Token::Atom(v) => Token::NotAtom(v.clone()),
        }
    }
}

pub trait HasLambda {
    fn has_lambda(&self) -> bool;
}

impl<E> HasLambda for Vec<Token<E>> {
    fn has_lambda(&self) -> bool {
        self.iter()
            .filter(|t| match t {
                Token::NoAtom => true,
                _ => false,
            })
            .next()
            .is_some()
    }
}

pub trait First<I> {
    fn first(&self) -> Vec<Token<I>>;
}
