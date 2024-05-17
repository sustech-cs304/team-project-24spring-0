use crate::interface::parser::{ParserInstSet, ParserResult};
use std::fmt::Display;

pub trait Assembler<IS>: Send + Sync
where
    IS: ParserInstSet + InstructionSetTrait,
{
    fn assemble(&mut self, ast: ParserResult<IS>)
        -> Result<AssembleResult<IS>, Vec<AssemblyError>>;
    fn dump(&mut self, ast: ParserResult<IS>) -> Result<Memory, Vec<AssemblyError>>;
}

pub trait InstructionSetTrait {
    type Register;
    type Immediate;
}

pub struct AssembleResult<IS: ParserInstSet + InstructionSetTrait> {
    pub data: Vec<u32>,
    pub instruction: Vec<InstructionSet<IS>>,
}

pub struct InstructionSet<IS: ParserInstSet + InstructionSetTrait> {
    pub line_number: u64,
    pub instruction: Instruction<IS>,
    pub address: u32,
    pub code: u32,
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
            address: 0,
            code: 0,
        }
    }
}
