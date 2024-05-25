use std::fmt::Display;

use crate::{
    interface::parser::{ParserInstSet, ParserResult},
    types::middleware_types::AssemblerConfig,
};

pub trait Assembler<IS>: Send + Sync
where
    IS: ParserInstSet + InstructionSetTrait,
{
    fn assemble(&mut self, ast: ParserResult<IS>)
        -> Result<AssembleResult<IS>, Vec<AssemblyError>>;
    fn update_config(&mut self, config: &AssemblerConfig);
    fn dump(&mut self, ast: ParserResult<IS>) -> Result<Memory, Vec<AssemblyError>>;
}

pub trait InstructionSetTrait {
    type Register: Clone;
    type Immediate: Clone;
}

#[derive(Clone)]
pub struct AssembleResult<IS: ParserInstSet + InstructionSetTrait> {
    pub data: Vec<u32>,
    pub instruction: Vec<InstructionSet<IS>>,
}

#[derive(Clone)]
pub struct InstructionSet<IS: ParserInstSet + InstructionSetTrait> {
    pub line_number: u64,
    pub instruction: Instruction<IS>,
    pub address: u32,
    pub code: u32,
    pub basic: String,
}

#[derive(Clone)]
pub struct Instruction<IS: ParserInstSet + InstructionSetTrait> {
    pub operation: IS::Operator,
    pub operands: Vec<Operand<IS>>,
}

#[derive(Clone)]
pub enum Operand<IS: ParserInstSet + InstructionSetTrait> {
    Reg(IS::Register),
    Operator(IS::Immediate),
}

pub struct Memory {
    pub data: String,
    pub text: String,
}

#[derive(Debug)]
pub struct AssemblyError {
    pub line: usize,
    pub msg: String,
}

impl Display for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line:{} {}", self.line, self.msg)
    }
}

impl<IS: ParserInstSet + InstructionSetTrait> Instruction<IS> {
    pub fn new(operation: IS::Operator) -> Self {
        Instruction {
            operation,
            operands: vec![],
        }
    }
}

impl<IS: ParserInstSet + InstructionSetTrait> InstructionSet<IS> {
    pub fn new(instruction: Instruction<IS>) -> Self {
        InstructionSet {
            line_number: 0,
            instruction: instruction,
            address: 0,
            code: 0,
            basic: String::new(),
        }
    }
}

impl Default for AssemblerConfig {
    fn default() -> Self {
        Self {
            memory_map_limit_address: 0xffffffff,
            kernel_space_high_address: 0xffffffff,
            mmio_base_address: 0xffff0000,
            kernel_space_base_address: 0x80000000,
            user_space_high_address: 0x7fffffff,
            data_segment_limit_address: 0x7fffffff,
            stack_base_address: 0x7ffffffc,
            stack_pointer_sp: 0x7fffeffc,
            stack_limit_address: 0x10040000,
            heap_base_address: 0x10040000,
            dot_data_base_address: 0x10010000,
            global_pointer_gp: 0x10008000,
            data_segment_base_address: 0x10000000,
            dot_extern_base_address: 0x10000000,
            text_limit_address: 0x0ffffffc,
            dot_text_base_address: 0x00400000,
        }
    }
}
