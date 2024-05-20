use super::cpu::CPU;
use crate::{
    interface::{
        assembler::{AssembleResult, Instruction, InstructionSet, InstructionSetTrait},
        simulator::Simulator,
    },
    modules::riscv::{
        basic::{assembler::register, interface::parser::RISCV},
        rv32i::constants::{RV32IInstruction, RV32IRegister},
    },
    types::middleware_types::SyscallDataType,
};
pub struct RiscVSimulator {
    pub cpu: CPU,
}

impl<EXR, REG, ERR> Simulator<EXR, REG, ERR> for RiscVSimulator {
    fn load_inst(&mut self, ir: &EXR) -> Result<(), ERR> {
        unimplemented!();
    }

    fn run(&mut self) -> Result<REG, ERR> {
        unimplemented!();
    }

    fn debug(&mut self) -> Result<REG, ERR> {
        unimplemented!();
    }

    fn step(&mut self) -> Result<REG, ERR> {
        unimplemented!();
    }

    fn stop(&mut self) -> Result<(), ERR> {
        unimplemented!();
    }

    fn reset(&mut self) -> Result<REG, ERR> {
        unimplemented!();
    }

    fn undo(&mut self) -> Result<REG, ERR> {
        unimplemented!();
    }

    fn set_breakpoint(&mut self, line_number: u64) -> Result<(), ERR> {
        unimplemented!();
    }

    fn remove_breakpoint(&mut self, line_number: u64) -> Result<(), ERR> {
        unimplemented!();
    }

    fn syscall_input(&mut self, input: SyscallDataType) {
        unimplemented!();
    }
}
