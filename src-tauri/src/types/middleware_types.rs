use crate::interface::{
    assembler::Assembler, parser::Parser, simulator::Simulator, storage::MFile,
};


use serde::Serialize;
use crate::modules::riscv::basic::interface::parser::RISCV;

pub struct Tab {
    pub text: Box<dyn MFile<String>>,
    pub parser: Box<dyn Parser<RISCV>>,
    //pub assembler: Box<dyn Assembler<i32, i32, i32, i32>>,
    //pub simulator: Box<dyn Simulator<i32, i32, i32, i32>>,
}

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

