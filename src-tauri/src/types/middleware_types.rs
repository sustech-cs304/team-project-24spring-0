use crate::interface::{
    assembler::Assembler, parser::Parser, simulator::Simulator, storage::MFile,
};

use ropey::Rope;

pub struct Tab {
    text: Box<dyn MFile<Rope, String, String>>,
    parser: Box<dyn Parser<Rope, crate::modules::riscv::basic::interface::parser::RISCV>>,
    assembler: Box<dyn Assembler<i32, i32, i32, i32>>,
    simulator: Box<dyn Simulator<i32, i32, i32, i32>>,
}

use std::{collections::HashMap, sync::Mutex};

pub struct TabMap {
    pub tabs: Mutex<HashMap<String, Tab>>,
}

pub mod constants {
    pub enum Lint {
        Info,
        Lint,
        Warn,
        Error,
    }

    pub enum AssemblerOp {
        Assemble,
        Dump,
        DumpAs,
    }

    pub enum SimulatorOp {
        Run,
        Debug,
        RunStep,
        Redo,
    }

    pub enum FileOp {
        Save,
        SaveAs,
        Open,
        Close,
    }

    pub enum WebSocketOp {
        RefreshText,
    }
}
