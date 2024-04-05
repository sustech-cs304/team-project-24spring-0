use crate::interface::{
    assembler::Assembler, parser::Parser, simulator::Simulator, storage::MFile,
};
use ropey::Rope;

pub struct Tab {
    text: Box<dyn MFile<Rope, String, String>>,
    parser: Box<dyn Parser<Rope, i32, i32, i32>>,
    assembler: Box<dyn Assembler<i32, i32, i32, i32>>,
    simulator: Box<dyn Simulator<i32, i32, i32, i32>>,
}

use std::collections::HashMap;
pub type TabMap = HashMap<String, Tab>;
