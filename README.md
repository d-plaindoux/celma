# Celma 

[![Build Status](https://travis-ci.org/d-plaindoux/celma.svg?branch=master)](https://travis-ci.org/d-plaindoux/celma)

[Celma ("k")noun "channel" (KEL) in Quenya](https://www.elfdict.com/w/kelma)

Celma is a generalised parser combinator implementation. Generalised means not an implementation restricted to a stream of characters.

## Overview

is the capability to design parser based on pipelined parsers and separate parsers regarding their semantic level.

# [WIP] Parsec meta language

## Grammar
In order to have a seamless parser definition a dedicated `proc_macro` is designed.

```
parser     ::= (binding? atom)+ occurrence? additional? transform?
binding    ::= IDENT '='
occurrence ::= ("*" | "+" | "?")
additional ::= "|"? parser
transform  ::= "=>" '{' rust code '}'
atom       ::= '(' parser ')' | '{' rust code '}'
```

##  Using the meta language

Therefore a parser can be defined using this meta-language.

```
let DQUOTE = char('"');
let NOT_DQUOTE = not_char('"');
let parser = parsec!( {DQUOTE} s={NOT_DQUOTE}* {DQUOTE} => { TkString(s) } );
```

## A Full Example: JSON

```
//--------------------------------------------------------------------
// Atoms
//--------------------------------------------------------------------
let STRING    = delimited_string();
let NUMBER    = number();
//--------------------------------------------------------------------
// Parsing rules
//--------------------------------------------------------------------
let string    = parsec!( n={STRING}              => { TKString(n) } );
let integer   = parsec!( n={NUMBER}              => { TKNumber(n) } );
let null      = parsec!( "null"                  => { TKNull } );
let boolean   = parsec!( b =("true"|"false")     => { TKBool(b) } );
let array     = parsec!( '[' s={json()}* ']'     => { TkArray(s) } );
let attribute = parsec!( n=STRING ":" v={json()} => { (n,v) } );
let object    = parsec!( '{' s={attributes}* '}' => { TkObject(s) } );
fn json<'a, S:'a>() -> impl Parse<'a, JSon, S> + Combine<JSon> + 'a {  
   parsec!( integer|string|null|boolean|array|object|attribute )
}
//--------------------------------------------------------------------
```

# License

Copyright 2019 Didier Plaindoux.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
