use crate::modules::riscv::rv32i::constants::{RISCVImmediate, RV32IInstruction, RV32IRegister};

pub trait Assembler<IN, OUT, SET, ERR>: Send + Sync {
    fn assemble(&mut self, ast: &IN) -> Result<OUT, ERR>;
    fn dump(&self, ast: &IN) -> Result<String, ERR>;
    fn update_setting(&mut self, settings: &SET) -> Result<bool, String>;
}

pub struct Instruction {
    pub op: RV32IInstruction,
    pub ins: Vec<Operand>,
}

pub enum Operand {
    Reg(RV32IRegister),
    Operator(RISCVImmediate),
}
