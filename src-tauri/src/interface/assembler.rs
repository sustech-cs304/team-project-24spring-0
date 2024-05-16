use std::fmt::Display;

use crate::interface::parser::{ParserInstSet, ParserResult};

pub trait Assembler<IS>: Send + Sync
where
    IS: ParserInstSet + InstructionSetTrait,
{
    fn assemble(
        &mut self,
        ast: ParserResult<IS>,
    ) -> Result<Vec<InstructionSet<IS>>, Vec<AssemblyError>>;
    fn dump(&mut self, ast: ParserResult<IS>) -> Result<Memory, Vec<AssemblyError>>;
}

pub trait InstructionSetTrait {
    type Register;
    type Immediate;
}

pub struct InstructionSet<IS: ParserInstSet + InstructionSetTrait> {
    pub line_number: u64,
    pub instruction: Instruction<IS>,
}

pub struct Instruction<IS: ParserInstSet + InstructionSetTrait> {
    pub operation: IS::Operator,
    pub operands: Vec<Operand<IS>>,
}

pub enum Operand<IS: ParserInstSet + InstructionSetTrait> {
    Reg(IS::Register),
    Operator(IS::Immediate),
}

pub struct Memory {
    pub data: String,
    pub text: String,
}

#[derive(Debug)]
pub struct AssemblyError {
    pub line: usize,
    pub msg: String,
}

impl Display for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line:{} {}", self.line, self.msg)
    }
}

impl<IS: ParserInstSet + InstructionSetTrait> Instruction<IS> {
    pub fn new(operation: IS::Operator) -> Self {
        Instruction {
            operation,
            operands: vec![],
        }
    }
}

impl<IS: ParserInstSet + InstructionSetTrait> InstructionSet<IS> {
    pub fn new(instruction: Instruction<IS>) -> Self {
        InstructionSet {
            line_number: 0,
            instruction: instruction,
        }
    }
}
