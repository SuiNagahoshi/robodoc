
//#[derive(Clone, Debug)]
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Token {
    FileName(String),
    Brief(String),
    Author(String),
    Date(String),
    Details(String),
    Kind(Kind),
    Param(String, String),
    Return(String, String),
    Err(String)//一応
}

//#[derive(Clone, Debug)]
#[derive(Hash, Eq, PartialEq, Debug)]
enum Kind {
    Fn,
    Cls,
    Srt,
    Enm,
    Val(String, String),
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum BlockType {
    MultiLine,
    SingleLine,
}
/*
pub trait Tokenizer {
    fn tokenize() -> Result<Token, String>;
}
impl Tokenizer for Token {
    fn tokenize(&self) -> Result<Token, String> {

    }
}

impl Token {
    fn split_token(c: &str) -> Token {
        let t: Vec<&str> = c.splitn(3, ' ').collect();
    }
}*/
/*
impl fmt::Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Kind::Fn => "Fn",
            Kind::Cls => "Cls",
            Kind::Srt => "Srt",
            Kind::Enm => "Enm",
            Kind::Val => "Val",
            _ => "error",
        };
        write!(f, "{}",  s)
    }
}*/