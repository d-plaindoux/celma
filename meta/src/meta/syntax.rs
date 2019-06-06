#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ASTParsec {
    PSequence(Box<ASTParsec>, Box<ASTParsec>),
    PChoice(Box<ASTParsec>, Box<ASTParsec>),
    POptional(Box<ASTParsec>),
    PRepeat(bool, Box<ASTParsec>),
    PBind(String, Box<ASTParsec>),
    PCode(String),
    PMap(Box<ASTParsec>, String),
}
