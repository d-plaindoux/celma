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
    PAtom(char),                    // Single char
    PAtoms(Vec<char>),              // Char sequence
    PBind(String, Box<ASTParsec>),  // Variable
    PCode(String),                  // Production
    PMap(Box<ASTParsec>, String),   // Remove?
    PNot(Box<ASTParsec>),           // ?
    PCheck(Box<ASTParsec>),         // No capture

    -- Pre-normalization

    PN : ASTParsec -> (string -> ASTParsec) -> string list -> ASTGrammar

                     / Var(n)               if n in l
    PN[PIdent(n)]gl = {
                     \ mu(n,PN[g(n)]g(n::l) otherwise

    PN[PSequence(T1,T2]]gl  = Seq(PN[T1]gl,PN[T2]gl)
    PN[PChoice(T1,T2)]gl    = Choice(PN[T1]gl,PN[T2]gl)
    PN[PRepeat(false, T)]gl = Choice(PN[T]gl,PN[PRepeat(true, T)]gl)

    PN[PRepeat(true, T)]gl  = PN[PChoice(PRepeat(false, T),PEpsilon)]gl
    PN[POptional(T)]gl      = PN[PChoice(T,PEpsilon)]gl
    PN[PTry(T)gl]           = PN[T]gl
 */

