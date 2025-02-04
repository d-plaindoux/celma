## Celma Lang

Celma is a embedded language in Rust targeting simple parser construction. 
As already explained in the main README such language is processes during
the Rust compilation stage.

### V0

In the V0 the compilation is a direct style Parsec generation without any 
optimisations. 

### V1

This version target an aggressive and an efficient parser compilation. For this
purpose the compilation follows a traditional control ans data flow and was 
mainly inspired by the paper [A Typed, Algebraic Approach to Parsing](https://www.cl.cam.ac.uk/~jdy22/papers/a-typed-algebraic-approach-to-parsing.pdf)

#### Abstract syntax tree

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ASTParsec {
    PIdent(String),
    PChar(char),
    PString(String),
    PBind(String, Box<ASTParsec>),
    PMap(Box<ASTParsec>, String),
    PSequence(Box<ASTParsec>, Box<ASTParsec>),
    PChoice(Box<ASTParsec>, Box<ASTParsec>),
    PNot(Box<ASTParsec>),
    PTry(Box<ASTParsec>),
    PCheck(Box<ASTParsec>),
    POptional(Box<ASTParsec>),
    PRepeat(bool, Box<ASTParsec>),
    PLookahead(Box<ASTParsec>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ASTParsecRule {
    pub public: bool,
    pub name: String,
    pub input: String,
    pub returns: String,
    pub rule: Box<ASTParsec>,
}
```