use crate::interface::{
    assembler::Assembler, parser::Parser, simulator::Simulator, storage::MFile,
};

use ropey::Rope;

pub struct Tab {
    pub text: Box<dyn MFile<String>>,
    pub parser: Box<dyn Parser<Rope, crate::modules::riscv::basic::interface::parser::RISCV>>,
    pub assembler: Box<dyn Assembler<i32, i32, i32, i32>>,
    pub simulator: Box<dyn Simulator<i32, i32, i32, i32>>,
}

use std::{collections::HashMap, sync::Mutex};

pub struct TabMap {
    pub tabs: Mutex<HashMap<String, Tab>>,
}

pub struct CurTabName {
    pub name: Mutex<String>,
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
