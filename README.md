# Celma 

[![Build Status](https://travis-ci.org/d-plaindoux/celma.svg?branch=master)](https://travis-ci.org/d-plaindoux/celma)

[Celma ("k")noun "channel" (KEL) in Quenya](https://www.elfdict.com/w/kelma)

Celma is a generalised parser combinator implementation. Generalised means not an implementation restricted to a stream of characters.

## Overview

Genealization is the capability to design parser based on pipelined parsers and separate parsers regarding their semantic level.

# [WIP] Parsec meta language

## Grammar
In order to have a seamless parser definition two dedicated `proc_macro` are designed.

```rust
parsec_rules = ("let" ident ':' '{' rust_code '}' "=" parsec)+
parsec       = binding? atom occurrence? additional? transform?
binding      = ident '='
occurrence   = ("*" | "+" | "?")
additional   = "|"? parser
transform    = "->" '{' rust_code '}'
atom         = '(' parser ')' | CHAR | STRING | ident | '{' rust_code '}' | '^' atom
ident        = [a..zA..Z]+ - {"let"}
```

##  Using the meta language

Therefore a parser can be defined using this meta-language.

```rust
let parser = parsec!( '"' s=(^'"')* '"' -> { TkString(s) } );
```

## A Full Example: JSON

```rust
parsec_rules!(
    let json:{()}    = S (string | null | boolean | array | object | number) S
    let number:{()}  = NUMBER                           -> {}
    let string:{()}  = STRING                           -> {}
    let null:{()}    = "null"                           -> {}
    let boolean:{()} = ("true"|"false")                 -> {}
    let array:{()}   = '[' (json (',' json)*)? ']'      -> {}
    let object:{()}  = '{' (attr (',' attr)*)? '}'      -> {}
    let attr:{()}    = S STRING S ":" json              -> {}
    
    let STRING:{()}  = '"' (^'"')* '"'                  -> {}
    let NUMBER:{()}  = INT ('.' NAT)? (('E'|'e') INT)?  -> {}
    let INT:{()}     = ('-'|'+')? NAT                   -> {}
    let NAT:{()}     = digit+                           -> {}
    let S:{()}       = space*                           -> {}
);
```

## Bootstrap scenario

### Stage 1

The Celma parser v0 is written with parser combinators.

### Stage 2

The Celma parser V1 is written using the Celma parser V0

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
