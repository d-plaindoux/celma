/*
 * Copyright 2019-2025 Didier Plaindoux
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
    use celma_lang_v0_ast::syntax::ASTParsec::{PAtom, PIdent};
    use celma_lang_v1::token::First;
    use celma_lang_v1::token::Token::{AllAtom, Atom};

    #[test]
    fn it_compute_first_for_ident() {
        assert_eq!(vec![AllAtom::<char>], PIdent(String::from("test")).first());
    }

    #[test]
    fn it_compute_first_for_char() {
        assert_eq!(vec![Atom('a')], PAtom('a').first());
    }
}
