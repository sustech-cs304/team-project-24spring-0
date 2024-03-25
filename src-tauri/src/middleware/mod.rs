mod backend_test;
mod frontend_test;
pub mod implementation;

mod middleware_constants {
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
