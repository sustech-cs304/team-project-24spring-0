use ropey::Rope;

use crate::interface::{
    assembler::Assembler, file_io::MFile, parser::Parser, simulator::Simulator,
};
use std::{collections::HashMap, sync::Mutex};

struct Tabs {
    stores: Mutex<HashMap<String, Box<Tab>>>,
}

struct Tab {
    text: Box<dyn MFile<Rope, String, String>>,
    parser: Box<dyn Parser<Rope, i32, i32, i32>>,
    assembler: Box<dyn Assembler<i32, i32, i32, i32, i32>>,
    simulator: Box<dyn Simulator<i32, i32, i32, i32>>,
}
