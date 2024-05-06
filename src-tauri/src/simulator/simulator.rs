use crate::interface::assembler::{Instruction, Operand};
use crate::interface::simulator::Simulator;
use crate::simulator::cpu::CPU;
use crate::types::middleware_types::SyscallDataType;

struct RiscVSimulator {
    // pc: u32,
    // memory: Vec<u8>,
    // registers: [u32; 32],
    // instructions: Vec<Instruction>,
    cpu: CPU,
}

impl<EXR, REG, ERR> Simulator<EXR, REG, ERR> for RiscVSimulator {
    fn load_inst(&mut self, ir: &EXR) -> Result<bool, ERR> {
        unimplemented!()
    }

    fn run(&mut self) -> Result<REG, ERR> {
        unimplemented!()
    }

    fn step(&mut self) -> Result<REG, ERR> {
        unimplemented!();
    }

    fn reset(&mut self) -> Result<REG, ERR> {
        unimplemented!();
    }

    fn redo(&mut self) -> Result<REG, ERR> {
        unimplemented!();
    }

    fn set_breakpoint(&mut self, line_number: u64) -> Result<bool, ERR> {
        unimplemented!();
    }

    fn remove_breakpoint(&mut self, line_number: u64) -> Result<bool, ERR> {
        unimplemented!();
    }

    fn syscall_input(&mut self, input: SyscallDataType) {
        unimplemented!();
    }
}
