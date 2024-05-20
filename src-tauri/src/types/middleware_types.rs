use std::{collections::HashMap, sync::Mutex};

use ropey::Rope;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumMessage};

use crate::{
    interface::{
        assembler::Assembler,
        parser::{Parser, ParserResult},
        storage::MFile,
    },
    modules::riscv::basic::interface::parser::RISCV,
    remote::server::RpcServerImpl,
};

//TODO: add simulator and assembler as member
pub struct Tab {
    pub text: Box<dyn MFile<Rope, String>>,
    pub parser: Box<dyn Parser<RISCV>>,
    pub assembler: Box<dyn Assembler<RISCV>>,
    //pub simulator: Box<dyn Simulator<i32, i32, i32, i32>>,
    pub data_return_range: (u64, u64),
    pub assembly_cache: AssembleCache,
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
pub struct CursorPosition {
    pub row: u64,
    pub col: u64,
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
    pub data: Vec<Data>,
    pub text: Vec<Text>,
}

#[derive(Clone, Serialize)]
pub struct Text {
    pub line: u64,
    pub address: u32,
    pub code: u32,
    pub basic: String,
}

pub type Data = u32;

#[derive(Clone, Serialize)]
pub struct AssembleError {
    pub line: u64,
    pub column: u64,
    pub msg: String,
}

#[derive(Default)]
pub struct AssembleCache {
    pub code: String,
    pub parser_cache: Option<ParserResult<RISCV>>,
    pub parser_result: Option<Vec<AssembleError>>,
    pub assembler_result: Option<AssembleResult>,
}

#[derive(Clone, Serialize)]
pub struct SimulatorResult {
    pub success: bool,
    pub has_current_text: bool,
    pub current_text: u64,
    pub registers: Vec<Register>,
    pub data: Vec<Data>,
    pub has_message: bool,
    pub message: String,
}

#[derive(Clone, Serialize)]
pub struct Register {
    pub name: String,
    pub number: String,
    pub value: u64,
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
    pub memory_map_limit_address: u32,
    pub kernel_space_high_address: u32,
    pub mmio_base_address: u32,
    pub kernel_space_base_address: u32,
    pub user_space_high_address: u32,
    pub data_segment_limit_address: u32,
    pub stack_base_address: u32,
    pub stack_pointer_sp: u32,
    pub stack_limit_address: u32,
    pub heap_base_address: u32,
    pub dot_data_base_address: u32,
    pub global_pointer_gp: u32,
    pub data_segment_base_address: u32,
    pub dot_extern_base_address: u32,
    pub text_limit_address: u32,
    pub dot_text_base_address: u32,
}

impl AssemblerConfig {
    pub fn new() -> Self {
        AssemblerConfig {
            memory_map_limit_address: 0x80000000,
            kernel_space_high_address: 0x80000000,
            mmio_base_address: 0x40000000,
            kernel_space_base_address: 0x80000000,
            user_space_high_address: 0x40000000,
            data_segment_limit_address: 0x80000000,
            stack_base_address: 0x7FFFFFFF,
            stack_pointer_sp: 0x7FFFFFFF,
            stack_limit_address: 0x7FFFFFF0,
            heap_base_address: 0x10000000,
            dot_data_base_address: 0x10000000,
            global_pointer_gp: 0x10000000,
            data_segment_base_address: 0x10000000,
            dot_extern_base_address: 0x10000000,
            text_limit_address: 0x10000000,
            dot_text_base_address: 0x10000000,
        }
    }
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

#[derive(Deserialize)]
pub enum FileOperation {
    Insert = 0,
    Delete = 1,
    Cover = 2,
}
