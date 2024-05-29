use std::{error::Error, net::Ipv4Addr, path::PathBuf, str::FromStr, sync::Mutex};

use once_cell::sync::Lazy;
use tauri::async_runtime::block_on;

use crate::{
    interface::remote::RpcClient,
    modules::riscv::basic::interface::{
        assembler::RiscVAssembler,
        parser::{RISCVExtension, RISCVParser},
    },
    remote::{client::RpcClientImpl, server::RpcServerImpl, utils::get_free_port},
    simulator::simulator::RISCVSimulator,
    storage::rope_store,
    tests::remote::{MAX_PORT_RETRY, TEST_FILE_NAME, TEST_PASSWD},
    types::middleware_types::{Tab, TabMap},
    utility::ptr::Ptr,
};

static TABMAP: Lazy<Mutex<Option<TabMap>>> = Lazy::new(|| Mutex::new(None));

pub fn init_test_server(content: &str) -> Result<RpcServerImpl, Box<dyn Error>> {
    if TABMAP.lock().unwrap().as_ref().is_none() {
        {
            let mut static_lock = TABMAP.lock().unwrap();
            *static_lock = Some(TabMap::default());
        }
        let content = rope_store::Text::from_str(
            PathBuf::from_str(TEST_FILE_NAME).unwrap().as_path(),
            content,
        );
        let mut static_lock = TABMAP.lock().unwrap();
        let static_tabmap = static_lock.as_mut().unwrap();
        let mut static_tab = static_tabmap.tabs.lock().unwrap();
        let tab = Tab {
            text: Box::new(content),
            parser: Box::new(RISCVParser::new(&vec![RISCVExtension::RV32I])),
            assembler: Box::new(RiscVAssembler::new()),
            simulator: Box::new(RISCVSimulator::new(TEST_FILE_NAME)),
            assembly_cache: Default::default(),
        };
        static_tab.insert(TEST_FILE_NAME.to_string(), tab);
    }
    let mut server = RpcServerImpl::default();

    server.change_password(TEST_PASSWD);
    let _ = server
        .set_port(get_free_port(Ipv4Addr::from_str("0.0.0.0").unwrap(), MAX_PORT_RETRY).unwrap());
    server
        .start_server(
            TEST_FILE_NAME.to_string(),
            Ptr::new(TABMAP.lock().unwrap().as_ref().unwrap()),
        )
        .unwrap();
    Ok(server)
}

pub fn init_test_client(port: u16) -> Result<RpcClientImpl, Box<dyn Error>> {
    let mut client = RpcClientImpl::default();
    client
        .set_server_addr(format!("0.0.0.0:{}", port).parse().unwrap())
        .unwrap();
    block_on(client.connect()).unwrap();
    Ok(client)
}
