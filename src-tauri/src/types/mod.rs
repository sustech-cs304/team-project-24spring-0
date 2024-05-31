pub mod menu_types;
pub mod middleware_types;
pub mod rpc_types;

pub type ResultVoid = Result<(), Box<dyn std::error::Error>>;
