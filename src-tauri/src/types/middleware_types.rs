use crate::interface::assembler::{AssembleResult as AssemblerResult, Assembler};
use crate::interface::parser::{Parser, ParserResult};
use crate::interface::simulator::Simulator;
use crate::interface::storage::MFile;
use crate::modules::riscv::basic::interface::parser::RISCV;
use crate::remote::server::RpcServerImpl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use strum_macros::{Display, EnumMessage};

//TODO: add simulator and assembler as member
pub struct Tab {
    pub text: Box<dyn MFile<String>>,
    pub parser: Box<dyn Parser<RISCV>>,
    pub assembler: Box<dyn Assembler<RISCV>>,
    //pub simulator: Box<dyn Simulator<i32, i32, i32, i32>>,
    pub parser_result: (Option<ParserResult<RISCV>>, Option<Vec<AssembleError>>),
    pub assemble_result: (Option<AssemblerResult<RISCV>>, Option<AssembleResult>),
}

#[derive(Default)]
pub struct TabMap {
    pub tabs: Mutex<HashMap<String, Tab>>,
    pub rpc_server: Mutex<RpcServerImpl>,
}

pub struct CurTabName {
    pub name: Mutex<String>,
}

#[derive(Clone, Serialize, Default)]
pub struct Optional {
    pub success: bool,
    pub message: String,
}

#[derive(Clone, Deserialize)]
pub struct TextPosition {
    pub row: u64,
    pub column: u64,
}

#[derive(Clone, Serialize)]
pub enum AssembleResult {
    Success(AssembleSuccess),
    Error(Vec<AssembleError>),
}

#[derive(Clone, Serialize)]
pub enum DumpResult {
    Success(()),
    Error(Vec<AssembleError>),
}

#[derive(Clone, Serialize)]
pub struct AssembleSuccess {
    pub data: Vec<AssembleSuccessData>,
    pub text: Vec<AssembleSuccessText>,
}

#[derive(Clone, Serialize)]
pub struct AssembleSuccessText {
    pub line: u64,
    pub address: u32,
    pub code: u32,
    pub basic: String,
}

pub type AssembleSuccessData = u32;

#[derive(Clone, Serialize)]
pub struct AssembleError {
    pub line: u64,
    pub column: u64,
    pub msg: String,
}

#[derive(Clone, Serialize)]
pub struct SyscallOutput {
    pub filepath: String,
    pub data: String,
}

#[derive(Clone, Serialize)]
pub struct SyscallRequest {
    pub path: String,
    pub syscall: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AssemblerConfig {
    memory_map_limit_address: u64,
    kernel_space_high_address: u64,
    mmio_base_address: u64,
    kernel_space_base_address: u64,
    user_space_high_address: u64,
    data_segment_limit_address: u64,
    stack_base_address: u64,
    stack_pointer_sp: u64,
    stack_limit_address: u64,
    heap_base_address: u64,
    dot_data_base_address: u64,
    global_pointer_gp: u64,
    data_segment_base_address: u64,
    dot_extern_base_address: u64,
    text_limit_address: u64,
    dot_text_base_address: u64,
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
