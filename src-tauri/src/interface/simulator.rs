use super::BasicOp;

//TODO define the status and error type
type MemStatus = String;
type Error = String;

pub trait Simulator<ER, REG, MS, ERR>: BasicOp<MS, ERR> {
    fn load_inst(&mut self, ir: &ER) -> bool;
    fn run(&mut self) -> bool;
    fn step(&mut self) -> bool;
    fn reset(&mut self) -> bool;
    fn redo(&mut self) -> bool;
    fn set_breakpoint(&mut self, addr: u64) -> bool;
    fn remove_breakpoint(&mut self, addr: u64) -> bool;
    fn set_register(&mut self, reg: REG, value: u64) -> bool;
}
