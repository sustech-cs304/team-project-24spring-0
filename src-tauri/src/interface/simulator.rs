pub trait Simulator<EXR, SYSC, REG, ERR> {
    fn load_inst(&mut self, ir: &EXR) -> Result<bool, ERR>;
    fn run(&mut self) -> Result<REG, ERR>;
    fn step(&mut self) -> Result<REG, ERR>;
    fn reset(&mut self) -> Result<REG, ERR>;
    fn redo(&mut self) -> Result<REG, ERR>;
    fn set_breakpoint(&mut self, line_number: u64) -> Result<bool, ERR>;
    fn remove_breakpoint(&mut self, line_number: u64) -> Result<bool, ERR>;
    fn syscall_input(&mut self, input: SYSC);
}
