use crate::interface::simulator::Simulator;
use crate::simulator::cpu::*;

struct RiscVSimulator {
    pub cpu: Cpu,
}

impl<EXR, SYSC, REG, ERR> Simulator<EXR, SYSC, REG, ERR> for RiscVSimulator {
    fn load_inst(&mut self, ir: &EXR) -> Result<bool, ERR> {
        unimplemented!();
    }

    fn run(&mut self) -> Result<REG, ERR> {
        unimplemented!();
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

    fn syscall_input(&mut self, input: SYSC) {
        unimplemented!();
    }
}
