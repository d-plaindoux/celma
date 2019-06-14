# Celma 

[![Build Status](https://travis-ci.org/d-plaindoux/celma.svg?branch=master)](https://travis-ci.org/d-plaindoux/celma)
[![unstable](http://badges.github.io/stability-badges/dist/unstable.svg)](http://github.com/badges/stability-badges)

[Celma ("k")noun "channel" (KEL) in Quenya](https://www.elfdict.com/w/kelma)

Celma is a generalised parser combinator implementation. Generalised means not an implementation restricted to a stream of characters.

## Overview

Genealization is the capability to design parser based on pipelined parsers and separate parsers regarding their semantic level.

# Celma parser meta language

## Grammar
In order to have a seamless parser definition two dedicated `proc_macro` are designed.

```rust
parsec_rules = ("let" ident ':' '{' rust_code '}' "=" parsec)+
parsec       = binding? atom occurrence? additional? transform?
binding      = ident '='
occurrence   = ("*" | "+" | "?")
additional   = "|"? parser
transform    = "->" '{' rust_code '}'
atom         = alter? '(' parser ')' | CHAR | STRING | ident | '{' rust_code '}'
alter        = ("^"|"!"|"#")
ident        = [a..zA..Z]+ - {"let", "try", "check"}
```

##  Using the meta language

Therefore a parser can be defined using this meta-language.

```rust
let parser = parsec!( '"' s=(("\"" -> { '\"' })|^'"')* '"' -> { TkString(s) } );
```

## A Full Example: JSON

A [JSon parser](https://github.com/d-plaindoux/celma/blob/master/plugin/benches/json.rs) can be designed thanks to the Celma parser meta language.

### JSon abstract data type

```rust
#[derive(Clone)]
pub enum JSON {
    Number(f64),
    String(String),
    Null,
    Bool(bool),
    Array(Vec<JSON>),
    Object(Vec<(String, JSON)>),
}
```

### Transformation functions 

```rust
fn mk_vec<E>(a: Option<(E, Vec<E>)>) -> Vec<E> {
    if a.is_none() {
        Vec::new()
    } else {
        let (a, v) = a.unwrap();
        let mut r = v;
        r.insert(0, a);
        r
    }
}

fn mk_f64(a: Vec<char>) -> f64 {
    a.into_iter().collect::<String>().parse().unwrap()
}
```

### The JSon parser

```rust
parsec_rules!(
    //-------------------------------------------------------------------------
    let json:{JSON}          = S _=(string | null | boolean  | array | object | number) S
    //-------------------------------------------------------------------------
    let number:{JSON}        = f=NUMBER                                 -> {JSON::Number(f)}
    let string:{JSON}        = s=STRING                                 -> {JSON::String(s)}
    let null:{JSON}          = "null"                                   -> {JSON::Null}
    let boolean:{JSON}       = b=("true"|"false")                       -> {JSON::Bool(b=="true")}
    let array:{JSON}         = ('[' S a=(_=json _=(',' _=json)*)? ']')  -> {JSON::Array(mk_vec(a))}
    let object:{JSON}        = ('{' S a=(_=attr _=(',' _=attr)*)? '}')  -> {JSON::Object(mk_vec(a))}
    let attr:{(String,JSON)} = (S s=STRING S ":" j=json)                -> {(s,j)}
    //-------------------------------------------------------------------------
    let STRING:{String}      = delimited_string
    //-------------------------------------------------------------------------
    let NUMBER:{f64}         = c=#(INT ('.' NAT)? (('E'|'e') INT)?)     -> {mk_f64(c)}
    let INT:{()}             = ('-'|'+')? NAT                           -> {}
    let NAT:{()}             = digit+                                   -> {}
    //-------------------------------------------------------------------------
    let S:{()}               = space*                                   -> {}
    //-------------------------------------------------------------------------
);
```

## Bootstrap scenario

### Stage 1

The [Celma parser v0](https://github.com/d-plaindoux/celma/blob/master/meta/src/meta/parser.rs) is written with parser combinators

### [WIP] Stage 2

The Celma parser V1 is written using the Celma parser V0.

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
