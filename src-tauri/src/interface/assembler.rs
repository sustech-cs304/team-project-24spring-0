use crate::interface::parser::{ParserInstSet, ParserResult};
use crate::modules::riscv::basic::interface::parser::{ParserRISCVInstOp, ParserRISCVRegister};
use crate::modules::riscv::rv32i::constants::{RISCVImmediate, RV32IInstruction};

pub trait Assembler<IS>: Send + Sync
where
    IS: ParserInstSet,
{
    fn assemble(
        &mut self,
        ast: ParserResult<IS>,
    ) -> Result<Vec<InstructionSet>, Vec<AssemblyError>>;
    fn dump(&mut self, ast: ParserResult<IS>) -> Result<Memory, Vec<AssemblyError>>;
}

pub struct InstructionSet {
    pub line_number: u64,
    pub instruction: Instruction,
}

pub struct Instruction {
    pub operation: ParserRISCVInstOp,
    pub operands: Vec<Operand>,
}

pub enum Operand {
    Reg(ParserRISCVRegister),
    Operator(RISCVImmediate),
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

impl std::fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line:{} {}", self.line, self.msg)
    }
}

impl Instruction {
    pub fn new() -> Self {
        Instruction {
            operation: ParserRISCVInstOp::from(RV32IInstruction::Add),
            operands: vec![],
        }
    }
}

impl InstructionSet {
    pub fn new() -> Self {
        InstructionSet {
            line_number: 0,
            instruction: Instruction::new(),
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

impl std::fmt::Display for InstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:3} {:?}", self.line_number, self.instruction.operation)?;
        for ins in &self.instruction.operands {
            write!(f, "{}", ins.to_string()).expect("panic");
        }
        Ok(())
    }
}
