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
additional ::= ("~" | "|") parser
transform  ::= "=>" { rust code }
atom       ::= '(' parser ')' | CHAR | NUMBER | STRING | ^CHAR | { rust code }
```

##  Usage

Therefore a parser should define using this meta-language.

```
let DQUOTE = '"';
let parser = parsec!( {DQUOTE} s=^{DQUOTE}* {DQUOTE} => { TkString(s) } );
```

## Ful Example: JSON

```
let NULL      = "null";
let TRUE      = "true";
let FALSE     = "false";

let string    = parsec!( n=STRING                 => { TKString(n) } );
let integer   = parsec!( n=NUMBER                 => { TKNumber(n) } );
let null      = parsec!( {NULL}                   => { TKNull } );
let boolean   = parsec!( b =({TRUE}|{FALSE})      => { TKBool(b) } );
let array     = parsec!( '[' s=^{json}* ']'       => { TkArray(s) } );
let object    = parsec!( '{' s=^{attributes}* '}' => { TkObject(s) } );
let attribute = parsec!( n=STRING ":" v=json      => { (n,v) } );
let json      = parsec!( (integer|string|null|boolean|array|object|attribute) );
```

# License

Copyright 2019 D. Plaindoux.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
