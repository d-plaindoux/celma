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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ASTGrammar<A> {
    Epsilon(),
    Bottom(),
    Var(String),
    Token(A),
    Seq(Box<AST<A>>, Box<AST<A>>),
    Choice(Box<AST<A>>, Box<AST<A>>),
    Rec(String, Box<AST<A>>),
    // Extension for capture and transformation
    Bind(String, Box<AST<A>>),
    Map(Box<AST<A>>, String),
    // Negation for alternate paths
    Not(Box<AST<A>>),
}

/*
   PN : ASTParsec -> (string -> ASTParsec) -> string list -> ASTGrammar
   PN : ASTParsec -> (string -> ASTParsec) -> string list -> ASTGrammar

   PN[PIdent(n)]gl          = Var(n)               if n in l
                            | mu(n,PN[g(n)]g(n::l) otherwise
   PN[PAtom(c)]gl           = TokenChar(c)
   PN[PAtoms([])]gl         = Epsilon()
   PN[PAtoms(c::l)]gl       = Seq(Token(v),PN[PAtoms(c::l)]gl)
   PN[PEpsilon()]gl         = Epsilon()
   PN[PSequence(T1,T2]]gl   = Seq(PN[T1]gl,PN[T2]gl)
   PN[PChoice(T1,T2)]gl     = Choice(PN[T1]gl,PN[T2]gl)
   PN[PRepeat(T)]gl         = PN[T]gl|mu(n,Choice(Seq(PN[T]gl,Var(n)),Epsilon())) - fresh n not in l
   PN[PTry(T)gl]            = PN[T]gl
   PN[PCheck(T)gl]          = PN[T]gl
   PN[PBind(s,T)]gl         = Bind(s,PN[T]gl)
   PN[PMap(T,s)]gl          = Map(PN[T]gl,s)
   PN[PNot(T)]gl            = Not(PN[T]gl)
*/
