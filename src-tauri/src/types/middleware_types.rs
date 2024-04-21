use crate::interface::parser::ParserError;
use crate::interface::{
    assembler::Assembler, parser::Parser, simulator::Simulator, storage::MFile,
};
use crate::modules::riscv::basic::interface::parser::RISCV;
use serde::Serialize;
use std::{collections::HashMap, sync::Mutex};

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

pub enum SyscallDataType {
    Char(u8),
    String(Vec<u8>),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}
