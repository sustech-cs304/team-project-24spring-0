use crate::interface::parser::{
    ParserError, ParserInstSet, ParserResult, ParserResultData, ParserResultText, Pos,
};
use crate::modules::riscv::basic::interface::parser::{ParserRISCVInstOp, ParserRISCVRegister};
use crate::modules::riscv::rv32i::constants::{RISCVImmediate, RV32IInstruction, RV32IRegister};
use crate::storage::rope_store::Text;

pub trait Assembler<IS>: Send + Sync
where
    IS: ParserInstSet,
{
    fn assemble(
        &mut self,
        inst: &ParserResultText<IS>,
        index: usize,
    ) -> Result<Instruction, Vec<AssembleError>>;
    fn dump(
        &mut self,
        ast: Result<ParserResult<IS>, Vec<ParserError>>,
    ) -> Result<Memory, Vec<AssembleError>>;
}

pub struct Instruction {
    pub op: ParserRISCVInstOp,
    pub ins: Vec<Operand>,
}

pub enum Operand {
    Reg(ParserRISCVRegister),
    Operator(RISCVImmediate),
}

pub struct Memory {
    pub data: Vec<String>,
    pub text: Vec<String>,
}

#[derive(Debug)]
pub struct AssembleError {
    pub line: usize,
    pub msg: String,
}

impl std::fmt::Display for AssembleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line:{} {}", self.line, self.msg)
    }
}

impl Instruction {
    pub fn new() -> Self {
        Instruction {
            op: ParserRISCVInstOp::from(RV32IInstruction::Add),
            ins: vec![],
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Reg(reg) => {
                write!(f, " {:?}", reg)?;
                Ok(())
            }
            Operand::Operator(imm) => {
                write!(f, " {:?}", imm)?;
                Ok(())
            }
        }
    }
}
