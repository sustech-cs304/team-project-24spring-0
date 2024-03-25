use ropey::Rope;

use crate::interface::{
    assembler::Assembler, parser::Parser, simulator::Simulator, storage::MFile,
};
use std::{collections::HashMap, sync::Mutex};

pub struct Tab {
    text: Box<dyn MFile<Rope, String, String>>,
    parser: Box<dyn Parser<Rope, i32, i32, i32>>,
    assembler: Box<dyn Assembler<i32, i32, i32, i32>>,
    simulator: Box<dyn Simulator<i32, i32, i32, i32>>,
}
