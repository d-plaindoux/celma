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

pub enum ASTGrammar<A> {
    Epsilon,
    Token(A),
    Bottom(),
    Seq(Box<AST<A>>, Box<AST<A>>),
    Choice(Box<AST<A>>, Box<AST<A>>),
    Rec(String, Box<AST<A>>),
    Var(String)
}

/*
  -- Pre-normalization
  PN : ASTParsec -> ASTGrammar

  PN[PSequence(T1,T2]] = Seq(PN[T1],PN[T2])
  PN[PChoice(T1,T2]] = Choice(PN[T1],PN[T2])
  PN[PRepeat(true, T)] = Rec(a,Choice(Epsilon, Seq(PN[T],String(a)) // When a is not in FV(T)
  PN[PRepeat(false, T)] = Choice(PN[T],PN[PRepeat(true, T)])
  PN[POptional(T)] = Choice(Epsilon, PN[T])
 */
