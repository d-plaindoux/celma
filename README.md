# Celma 

[![stable](http://badges.github.io/stability-badges/dist/stable.svg)](http://github.com/badges/stability-badges)

[Celma ("k")noun "channel" (KEL) in Quenya](https://www.elfdict.com/w/kelma)

Celma is a generalised parser combinator implementation. Generalised means not an implementation restricted to a stream of characters.

## Overview

Generalization is the capability to design a parser based on pipelined parsers and separate parsers regarding their semantic level.

# Celma parser meta language

## Grammar
In order to have a seamless parser definition two dedicated `proc_macro` are designed:

```rust
parsec_rules = "pub"? "let" ident ('{' rust_type '}')? (':' '{' rust_type '}')? "=" parser)+
parser       = binding? atom occurrence? additional? transform?
```

```rust
binding      = ident '='
occurrence   = ("*" | "+" | "?")
additional   = "|"? parser
transform    = "->" '{' rust_code '}'
atom         = alter? '(' parser ')' | CHAR | STRING | ident
alter        = ("^"|"!"|"#"|"/")
ident        = [a..zA..Z][a..zA..Z0..9_]* - {"let"}
```

The `alter` is an annotation where:
- `^` allows the capability to recognize negation,
- `!` allows the capability to backtrack on failure and 
- `#` allows the capability to capture all chars.
- `/` allows the capability to lookahead without consuming scanned elements.

The `#` alteration is important because it prevents massive list construction in memory. 

##  Using the meta-language

Therefore, a parser can be defined using this meta-language.

```rust
let parser = parsec!( 
    ('{' v=^'}'* '}') -> { v.into_iter().collect::<String>() }
);
```

## A Full Example: JSON

A [JSon parser](https://github.com/d-plaindoux/celma/blob/master/macro/benches/json.rs#L61) can be designed thanks to the Celma parser meta language.

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

fn mk_string(a: Vec<char>) -> String {
    a.into_iter().collect::<String>()
}

fn mk_f64(a: Vec<char>) -> f64 {
    mk_string(a).parse().unwrap()
}
```

### The JSon parser

The JSon parser is define by six rules dedicated to `number`, `string`, `null`, `boolean`, `array` 
and `object`.

#### JSON Rules

```rust
parsec_rules!(
    let json:{JSON}          = S _=(string | null | boolean  | array | object | number) S
    let number:{JSON}        = f=NUMBER                                -> {JSON::Number(f)}
    let string:{JSON}        = s=STRING                                -> {JSON::String(s)}
    let null:{JSON}          = "null"                                  -> {JSON::Null}
    let boolean:{JSON}       = b=("true"|"false")                      -> {JSON::Bool(b=="true")}
    let array:{JSON}         = ('[' S a=(_=json _=(',' _=json)*)? ']') -> {JSON::Array(mk_vec(a))}
    let object:{JSON}        = ('{' S a=(_=attr _=(',' _=attr)*)? '}') -> {JSON::Object(mk_vec(a))}
    let attr:{(String,JSON)} = (S s=STRING S ":" j=json)
);
```

#### Basic rules and terminals

```rust
parsec_rules!(
    let STRING:{String} = delimited_string
    let NUMBER:{f64}    = c=#(INT ('.' NAT)? (('E'|'e') INT)?)    -> {mk_f64(c)}
    let INT             = ('-'|'+')? NAT                          -> {}
    let NAT             = digit+                                  -> {}
    let S               = space*                                  -> {}
);
```

## The expression parser thanks to pipelined parsers.

The previous parser mixes char analysis and high-level term construction. This can be done in a different manner since Celma is a generalized parser combinator implementation.

For instance a first parser dedicated to lexeme recognition can be designed. Then on top of this lexer an expression parser can be easily designed.  

### Tokenizer

A tokenizer consumes a stream of char and produces tokens.

```rust
parsec_rules!(
    let token:{Token}   = S _=(int|keyword) S
    let int:{Token}     = c=!(#(('-'|'+')? digit+)) -> {Token::Int(mk_i64(c))}
    let keyword:{Token} = s=('+'|'*'|'('|')')       -> {Token::Keyword(s)}
    let S               = space*                    -> {}
);
```

### Lexemes

The Lexeme parser recognizes simple token keywords. 

```rust
parsec_rules!(
    let PLUS{Token}   = {kwd('+')} -> {}
    let MULT{Token}   = {kwd('*')} -> {}
    let LPAREN{Token} = {kwd('(')} -> {}
    let RPAREN{Token} = {kwd(')')} -> {}
);
```

### Expression parser

The expression parser builds expression consuming tokens. For this purpose the stream type can be specified for each parser. If it's not the case the default one is `char`.
In the following example the declaration `expr{Token}:{Expr}` denotes a parser consuming a `Token` stream and producing an `Expr`. 

```rust
parsec_rules!(
    let expr{Token}:{Expr}     = (s=sexpr e=(_=oper _=expr)?) -> {mk_operation(s,e)}
    let oper{Token}:{Operator} = (PLUS                        -> {Operator::Plus})
                               | (MULT                        -> {Operator::Mult})
    let sexpr{Token}:{Expr}    = (LPAREN _=expr RPAREN)
                               | number
    let number{Token}:{Expr}   = i=kint                       -> {Expr::Number(i)}
);
```

### Expression parser in  action

```rust
let tokenizer = token();
let stream = ParserStream::new(&tokenizer, CharStream::new("1 + 2"));
let response = expr().and_left(eos()).parse(stream);

match response {
    Success(v, _, _) => assert_eq!(v.eval(), 3),
    _ => assert_eq!(true, false),
}
```

# Celma language internal design

Celma is an embedded language in Rust for building simple parsers.
The language is processed when Rust is compiled. To this end, we
identify two steps. The first is to analyse the language using a 
syntax analyser in a direct style. Then, this parser is invoked 
during the compilation phase, using a procedural macro dedicated 
to Rust to manage the language in Rust.

## V0

In the V0 the transpilation is a direct style Parsec generation without any
optimisations cf. [celma parser in direct style](https://github.com/d-plaindoux/celma/blob/master/lang/v0/parser/src/parser.rs). 

## V1

This version target an aggressive and an efficient parser compilation. For this
purpose the compilation follows a traditional control and data flow mainly inspired 
by the papers like [A Typed, Algebraic Approach to Parsing](https://www.cl.cam.ac.uk/~jdy22/papers/a-typed-algebraic-approach-to-parsing.pdf) 
and [Fusing Lexing and Parsing](https://www.cl.cam.ac.uk/~jdy22/papers/fusing-lexing-and-parsing.pdf).

First, we express [Celma in Celma](https://github.com/d-plaindoux/celma/blob/master/lang/v1/parser/src/parser.rs).
This gives us an AST denoting parsers expressed using the Celma language i.e. Celma(v1) thanks to Celma(v0).

### Normalisation

Work in progress

### Fusion

Work in progress

### Staging

Work in progress

# License

Copyright 2019-2025 Didier Plaindoux.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
