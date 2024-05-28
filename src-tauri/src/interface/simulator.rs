use crate::{
    interface::assembler::AssembleResult,
    modules::riscv::basic::interface::parser::RISCV,
    types::middleware_types::{AssemblerConfig, MemoryReturnRange},
};

pub trait Simulator: Send + Sync {
    fn load_inst(&mut self, inst: AssembleResult<RISCV>) -> Result<(), String>;
    fn get_raw_inst(&self) -> &Option<AssembleResult<RISCV>>;
    fn update_config(&mut self, config: &AssemblerConfig) -> Result<(), String>;
    fn run(&mut self) -> Result<(), String>;
    fn debug(&mut self) -> Result<(), String>;
    fn stop(&mut self) -> Result<(), String>;
    fn resume(&mut self) -> Result<(), String>;
    fn step(&mut self) -> Result<(), String>;
    fn reset(&mut self) -> Result<(), String>;
    fn undo(&mut self) -> Result<(), String>;
    fn set_breakpoint(&mut self, idx: usize) -> Result<(), String>;
    fn remove_breakpoint(&mut self, idx: usize) -> Result<(), String>;
    fn syscall_input(&mut self, input: &str) -> Result<(), String>;
    fn get_register(&self) -> &[u32];
    fn get_memory(&self) -> Vec<u32>;
    fn get_pc_idx(&self) -> Option<usize>;
    fn get_filepath(&self) -> &str;
    fn get_memory_return_range(&self) -> MemoryReturnRange;
    fn set_memory_return_range(&mut self, range: MemoryReturnRange) -> Result<(), String>;
}
