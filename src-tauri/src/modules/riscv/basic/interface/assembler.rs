use std::fmt::Display;

use crate::{
    interface::assembler::{InstructionSet, InstructionSetTrait, Operand},
    modules::riscv::basic::interface::parser::*,
};

impl InstructionSetTrait for RISCV {
    type Register = ParserRISCVRegister;
    type Immediate = RISCVImmediate;
}

impl Display for InstructionSet<RISCV> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "line_number: {:3}; address:0x{:08x}; code: 0x{:08x}; Instruction: {:?}",
            self.line_number, self.address, self.code, self.instruction.operation
        )?;
        write!(
            f,
            "{}",
            self.instruction
                .operands
                .iter()
                .map(|ins| ins.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
        .expect("panic");
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
