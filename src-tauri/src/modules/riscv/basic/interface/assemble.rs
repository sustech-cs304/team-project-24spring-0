use crate::interface::assembler::{InstructionSet, InstructionSetTrait, Operand};
use crate::modules::riscv::basic::interface::parser::*;
use std::fmt::Display;

pub use crate::modules::riscv::basic::assembler::assembler::RiscVAssembler;

impl InstructionSetTrait for RISCV {
    type Register = ParserRISCVRegister;
    type Immediate = RISCVImmediate;
}

impl Display for InstructionSet<RISCV> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:3} {:?}", self.line_number, self.instruction.operation)?;
        for ins in &self.instruction.operands {
            write!(f, "{}", ins.to_string()).expect("panic");
        }
        Ok(())
    }
}

impl Display for Operand<RISCV> {
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
