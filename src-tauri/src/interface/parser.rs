pub trait Parser<IS: ParserInstSet>: Send + Sync {
    fn parse(&mut self, code: &String) -> Result<ParserResult<IS>, Vec<ParserError>>;
}

// in crate::modules::[instruction_set]::basic::interface::parser
pub trait ParserInstSet {
    type Operator: Clone + std::fmt::Debug + PartialEq + Eq;
    type Operand: Clone + std::fmt::Debug + PartialEq + Eq;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos(pub usize, pub usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParserResult<IS: ParserInstSet> {
    pub data: Vec<ParserResultData>,
    pub text: Vec<ParserResultText<IS>>,
}

#[derive(Clone, Debug)]
pub struct ParserError {
    pub pos: Pos,
    pub msg: String,
}

pub type ParserResultData = u8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParserResultText<IS: ParserInstSet> {
    Text(ParserInst<IS>),
    Align(u8),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParserInst<IS: ParserInstSet> {
    pub line: usize,
    pub op: IS::Operator,
    pub opd: Vec<IS::Operand>,
}

impl<IS: ParserInstSet> std::fmt::Display for ParserResult<IS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "ParserResult {\n\
            data:\n",
        )?;
        f.write_str("     0x")?;
        for &d in &self.data {
            write!(f, "{:02x}", d)?;
        }
        f.write_str("\n")?;
        f.write_str("text:\n")?;
        for (i, t) in self.text.iter().enumerate() {
            write!(f, "{:3} {}\n", i + 1, t.to_string())?;
        }
        Ok(())
    }
}

impl<IS: ParserInstSet> std::fmt::Display for ParserResultText<IS> {
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
