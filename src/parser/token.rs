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
    Err(String), //一応
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