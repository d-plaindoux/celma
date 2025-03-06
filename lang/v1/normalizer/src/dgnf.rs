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

//
// Deterministic Greibach Normal Form
//

/*
Normalization of well-typed context-free expressions.

(epsilon)   N 𝜖     = 𝑛⇒{𝑛→𝜖}
(token)     N 𝑡     = 𝑛⇒{𝑛→𝑡}
(bot)       N ⊥     = 𝑛⇒∅
(seq)       N 𝑔1·𝑔2 = 𝑛⇒{𝑛→𝑁1 𝑛2 |𝑛1 →𝑁1 ∈𝐺1 }∪𝐺1 ∪𝐺2
                      where N 𝑔1 = 𝑛1 ⇒𝐺1 ∧N 𝑔2 = 𝑛2 ⇒𝐺2
(alt)       N 𝑔1∨𝑔2 = 𝑛⇒{𝑛→𝑁1 |𝑛1 →𝑁1 ∈𝐺1 }∪{𝑛→𝑁2 |𝑛2 →𝑁2 ∈𝐺2 }∪𝐺1 ∪𝐺2
                       where N 𝑔1 = 𝑛1 ⇒𝐺1 ∧N 𝑔2 = 𝑛2 ⇒𝐺2
(fix)       N 𝜇𝛼.𝑔  = 𝛼 ⇒{𝛼 →𝑁 |𝑛→𝑁 ∈𝐺} (1)
                        ∪{𝑛′→𝑁 𝑛′|𝑛′→𝛼𝑛′∈𝐺∧𝑛→𝑁 ∈𝐺} (2)
                        ∪𝐺\𝑛′→𝛼 𝑛′ (3)
                    where N 𝑔= 𝑛⇒𝐺
                        𝐺\𝑛′→𝛼 𝑛′ is 𝐺 with all 𝑛′→𝛼𝑛′removed for any 𝑛′and 𝑛′
(var)       N 𝛼     = 𝑛⇒{𝑛→𝛼}

Source: https://www.cl.cam.ac.uk/~jdy22/papers/fusing-lexing-and-parsing.pdf
 */
