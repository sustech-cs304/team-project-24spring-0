use crate::types::middleware_types::SyscallDataType;
use std::fmt::Display;

pub trait Simulator<EXR, REG, ERR>: Send + Sync {
    fn load_inst(&mut self, ir: &EXR) -> Result<(), ERR>;
    fn run(&mut self) -> Result<REG, ERR>;
    fn debug(&mut self) -> Result<REG, ERR>;
    fn step(&mut self) -> Result<REG, ERR>;
    fn stop(&mut self) -> Result<(), ERR>;
    fn reset(&mut self) -> Result<REG, ERR>;
    fn undo(&mut self) -> Result<REG, ERR>;
    fn set_breakpoint(&mut self, line_number: u64) -> Result<(), ERR>;
    fn remove_breakpoint(&mut self, line_number: u64) -> Result<(), ERR>;
    fn syscall_input(&mut self, input: SyscallDataType);
}

#[derive(Debug)]
pub struct SimulatesError {
    pub address: u32,
    pub msg: String,
}

impl Display for SimulatesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimulatesError at address 0x{:08X}: {}", self.address, self.msg)
    }
}
