use std::{net::Ipv4Addr, path::PathBuf, str::FromStr, sync::Mutex};

use once_cell::sync::Lazy;

use crate::{
    modules::riscv::basic::interface::{
        assembler::RiscVAssembler,
        parser::{RISCVExtension, RISCVParser},
    },
    remote::{client::RpcClientImpl, server::RpcServerImpl, utils::get_free_port},
    storage::rope_store,
    types::middleware_types::{Tab, TabMap},
    utility::ptr::Ptr,
};

static TEST_FILE_NAME: &str = "/foo/bar/test_file.txt";

static TEST_PASSWD: &str = "fdsfs";

static MAX_PROT_RETRY: usize = 1145;

static TABMAP: Lazy<Mutex<Option<TabMap>>> = Lazy::new(|| Mutex::new(None));

pub fn init_test_server(content: &str) -> Result<RpcServerImpl, String> {
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
            //simulator: Box::new(Default::default()),
            data_return_range: Default::default(),
            assembly_cache: Default::default(),
        };
        static_tab.insert(TEST_FILE_NAME.to_string(), tab);
    }
    let mut server = RpcServerImpl::default();
    let _ = server
        .start_server(
            TEST_FILE_NAME.to_string(),
            Ptr::new(TABMAP.lock().unwrap().as_ref().unwrap()),
        )
        .unwrap();
    server.change_password(TEST_PASSWD);
    let _ = server
        .set_port(get_free_port(Ipv4Addr::from_str("127.0.0.1").unwrap(), MAX_PROT_RETRY).unwrap());
    Ok(server)
}

pub fn init_test_client(port: u16) -> RpcClientImpl {
    todo!("init_test_client");
}
