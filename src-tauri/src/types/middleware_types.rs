use crate::interface::{
    assembler::Assembler, parser::Parser, simulator::Simulator, storage::MFile,
};

use crate::modules::riscv::basic::interface::parser::RISCV;
use serde::Serialize;

pub struct Tab {
    pub text: Box<dyn MFile<String>>,
    pub parser: Box<dyn Parser<RISCV>>,
    //pub assembler: Box<dyn Assembler<i32, i32, i32, i32>>,
    //pub simulator: Box<dyn Simulator<i32, i32, i32, i32>>,
}

use crate::interface::parser::ParserError;
use std::{collections::HashMap, sync::Mutex};

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
