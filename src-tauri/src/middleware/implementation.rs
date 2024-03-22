use ropey::Rope;

use crate::interface::{
    assembler::Assembler, file_io::MFile, parser::Parser, simulator::Simulator,
};
use std::{collections::HashMap, sync::Mutex};

struct Tabs {
    stores: Mutex<HashMap<String, Box<Tab>>>,
}

struct Tab {
    text: Box<dyn MFile<Rope>>,
    parser: Box<dyn Parser<String>>,
    assembler: Box<dyn Assembler<String, String, String>>,
    simulator: Box<dyn Simulator<String, String>>,
}
