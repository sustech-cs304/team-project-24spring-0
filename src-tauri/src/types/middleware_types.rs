use std::{collections::HashMap, sync::Mutex};

use serde::Serialize;
use strum_macros::{Display, EnumMessage};

use crate::{
    interface::{
        assembler::Assembler,
        parser::{Parser, ParserError},
        simulator::Simulator,
        storage::MFile,
    },
    modules::riscv::basic::interface::parser::RISCV,
};

pub struct Tab {
    pub text: Box<dyn MFile<String>>,
    pub parser: Box<dyn Parser<RISCV>>,
    //pub assembler: Box<dyn Assembler<i32, i32, i32, i32>>,
    //pub simulator: Box<dyn Simulator<i32, i32, i32, i32>>,
}

pub struct TabMap {
    pub tabs: Mutex<HashMap<String, Tab>>,
}

pub struct CurTabName {
    pub name: Mutex<String>,
}

#[derive(Clone, Serialize)]
pub struct Optional {
    pub success: bool,
    pub message: String,
}

#[derive(Clone, Serialize)]
pub struct AssembleResult {
    pub success: bool,
    pub error: Vec<ParserError>,
}

#[derive(Clone, Serialize)]
pub struct SyscallRequest {
    pub path: String,
    pub syscall: String,
}

#[derive(Clone, Serialize)]
pub struct AssemblerConfig {
    memory_map_limit_address: usize,
    kernel_space_high_address: usize,
    mmio_base_address: usize,
    kernel_space_base_address: usize,
    user_space_high_address: usize,
    data_segment_limit_address: usize,
    stack_base_address: usize,
    stack_pointer_sp: usize,
    stack_limit_address: usize,
    heap_base_address: usize,
    dot_data_base_address: usize,
    global_pointer_gp: usize,
    data_segment_base_address: usize,
    dot_extern_base_address: usize,
    text_limit_address: usize,
    dot_text_base_address: usize,
}

#[derive(EnumMessage, Display)]
pub enum SyscallDataType {
    #[strum(message = "Char")]
    Char(u8),
    #[strum(message = "String")]
    String(Vec<u8>),
    #[strum(message = "Int")]
    Int(i32),
    #[strum(message = "Long")]
    Long(i64),
    #[strum(message = "Float")]
    Float(f32),
    #[strum(message = "Double")]
    Double(f64),
}
