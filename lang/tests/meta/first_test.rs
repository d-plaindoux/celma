/*
 * Copyright 2019-2025 Didier Plaindoux
=======
 * Copyright 2019-2021 Didier Plaindoux
>>>>>>> 45ec19c (Manage compiler warnings and change License header)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[cfg(test)]
mod tests {
    use celma_lang::meta::syntax::ASTParsec::{PChar, PIdent};
    use celma_lang::meta::token::First;
    use celma_lang::meta::token::Token::{AllAtom, Atom};

    #[test]
    fn it_compute_first_for_ident() {
        assert_eq!(vec![AllAtom], PIdent(String::from("test")).first());
    }

    #[test]
    fn it_compute_first_for_char() {
        assert_eq!(vec![Atom('a')], PChar('a').first());
    }
}
