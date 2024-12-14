use crate::parser::parser::FileInfo;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum FileInfoToken {
    FileName(String),
    Description(String),
    Author(String),
    Date(String),
}
/*
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Token {
    FileName(String),
    Description(String),
    Author(String),
    Date(String),
    Details(String),
    Kind(Kind),
    Param(String, String),
    Return(String, String),
    Err(String), //一応
}
*/

#[derive(Debug, Clone)]
enum Kind {
    Fn,
    Cls,
    Srt,
    Enm,
    Val(String, String),
}

#[derive(Debug, Clone)]
pub enum LineType {
    MultiLine,
    SingleLine,
}
#[derive(Debug, Clone)]
pub enum BlockType {
    Raw(String),
    FileInfo(FileInfo),
}