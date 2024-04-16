use serde::Serialize;

pub trait Parser<IS>: Send + Sync
where
    IS: ParserInstSet,
{
    fn parse(&mut self, code: String) -> Result<ParserResult<IS>, Vec<ParserError>>;
}

// in crate::modules::[instruction_set]::basic::interface::parser
pub trait ParserInstSet
where
    Self::Operator: Clone + std::fmt::Debug,
    Self::Operand: Clone + std::fmt::Debug,
{
    type Operator;
    type Operand;
}

#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos(pub usize, pub usize);

#[derive(Clone, Debug)]
pub struct ParserResult<IS>
where
    IS: ParserInstSet,
{
    pub data: Vec<ParserResultData>,
    pub text: Vec<ParserResultText<IS>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ParserError {
    pub pos: Pos,
    pub msg: String,
}

#[derive(Clone, Debug)]
pub enum ParserResultData {
    Data(Vec<u8>),
    Align(u8),
}

#[derive(Clone, Debug)]
pub enum ParserResultText<IS>
where
    IS: ParserInstSet,
{
    Text(ParserInst<IS>),
    Align(u8),
}

#[derive(Clone, Debug)]
pub struct ParserInst<IS>
where
    IS: ParserInstSet,
{
    pub line: usize,
    pub op: IS::Operator,
    pub opd: Vec<IS::Operand>,
}
