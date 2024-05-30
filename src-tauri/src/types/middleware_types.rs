use std::{
    collections::{HashMap, LinkedList},
    sync::Mutex,
};

use ropey::Rope;
use serde::{Deserialize, Serialize};

use crate::{
    interface::{
        assembler::Assembler,
        parser::{Parser, ParserResult},
        simulator::Simulator,
        storage::MFile,
    },
    modules::riscv::basic::interface::parser::RISCV,
    remote::{client::RpcClientImpl, server::RpcServerImpl, ClientCursor, Modification},
};

pub type Cursor = LinkedList<ClientCursor>;

pub struct Tab {
    pub text: Box<dyn MFile<Rope, Modification, Cursor>>,
    pub parser: Box<dyn Parser<RISCV>>,
    pub assembler: Box<dyn Assembler<RISCV>>,
    pub simulator: Box<dyn Simulator>,
    pub assembly_cache: AssembleCache,
}

#[derive(Default)]
pub struct TabMap {
    pub tabs: Mutex<HashMap<String, Tab>>,
    pub rpc_server: Mutex<RpcServerImpl>,
    pub rpc_client: Mutex<RpcClientImpl>,
}

pub struct CurTabName {
    pub name: Mutex<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Optional {
    pub success: bool,
    pub message: String,
}

#[derive(Clone, Deserialize)]
pub struct CursorPosition {
    pub row: u64,
    pub col: u64,
}

/// both start and len are aligned by 4
#[derive(Clone, Copy, Deserialize)]
pub struct MemoryReturnRange {
    pub start: u64,
    pub len: u64,
}

impl Default for MemoryReturnRange {
    fn default() -> Self {
        Self {
            start: 0x10010000,
            len: 0x100,
        }
    }
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
    pub text: Vec<AssembleText>,
}

#[derive(Clone, Serialize)]
pub struct AssembleText {
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
pub struct SimulatorData {
    pub filepath: String,
    pub success: bool,
    pub paused: bool,
    pub has_current_text: bool,
    pub current_text: u64,
    pub registers: Vec<Register>,
    pub data: Vec<Data>,
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
    pub filepath: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AssemblerConfig {
    pub memory_map_limit_address: u64,
    pub kernel_space_high_address: u64,
    pub mmio_base_address: u64,
    pub kernel_space_base_address: u64,
    pub user_space_high_address: u64,
    pub data_segment_limit_address: u64,
    pub stack_base_address: u64,
    pub stack_pointer_sp: u64,
    pub stack_limit_address: u64,
    pub heap_base_address: u64,
    pub dot_data_base_address: u64,
    pub global_pointer_gp: u64,
    pub data_segment_base_address: u64,
    pub dot_extern_base_address: u64,
    pub text_limit_address: u64,
    pub dot_text_base_address: u64,
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

#[derive(Deserialize)]
pub enum FileOperation {
    Insert = 0,
    Delete = 1,
    Replace = 2,
}
