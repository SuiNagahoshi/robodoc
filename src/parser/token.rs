#[derive(Hash, Eq, PartialEq, Debug, Clone)]
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

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Kind {
    Fn,
    Cls,
    Srt,
    Enm,
    Val(String, String),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum BlockType {
    MultiLine,
    SingleLine,
}