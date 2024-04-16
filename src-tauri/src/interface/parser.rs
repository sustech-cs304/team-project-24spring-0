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

impl<IS> std::fmt::Display for ParserResult<IS>
where
    IS: ParserInstSet,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "ParserResult {\n\
            data:\n",
        )?;
        for (i, d) in self.data.iter().enumerate() {
            write!(f, "{:3} {}\n", i + 1, d.to_string())?;
        }
        f.write_str("text:\n")?;
        for (i, t) in self.text.iter().enumerate() {
            write!(f, "{:3} {}\n", i + 1, t.to_string())?;
        }
        Ok(())
    }
}

impl std::fmt::Display for ParserResultData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserResultData::Data(data) => {
                f.write_str("Data: 0x")?;
                for d in data {
                    write!(f, "{:02x}", d)?;
                }
                Ok(())
            }
            ParserResultData::Align(a) => write!(f, "Align {}", a),
        }
    }
}

impl<IS> std::fmt::Display for ParserResultText<IS>
where
    IS: ParserInstSet,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserResultText::Text(inst) => {
                write!(f, "{:3}: {:?}", inst.line + 1, inst.op)?;
                for opd in &inst.opd {
                    write!(f, " {:?}", opd)?;
                }
                Ok(())
            }
            ParserResultText::Align(a) => write!(f, "Align {}", a),
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "line:{} col:{} {}",
            self.pos.0 + 1,
            self.pos.1 + 1,
            self.msg
        )
    }
}
