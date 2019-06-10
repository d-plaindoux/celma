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

#![feature(proc_macro_hygiene)]

use celma_plugin::parsec_rules;

fn main() {
    parsec_rules!(
        let json:{JSon}    = number|string|null|boolean|array|object|attribute
        let string:{JSon}  = s={STRING}                       => { TKString(s) }
        let number:{JSon}  = n={NUMBER}                       => { TKNumber(n) }
        let null:{JSon}    = "null"                           => { TKNull      }
        let boolean:{JSon} = b=("true"|"false")               => { TKBool(b)   }
        let array:{JSon}   = '[' s=json* ']'                  => { TkArray(s)  }
        let object:{JSon}  = '{' s=(_=STRING ":" _=json)* '}' => { TkObject(s) }
    );
}
