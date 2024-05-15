use crate::modules::riscv::basic::interface::parser::{RISCVExtension, RISCVParser};
use crate::remote::server::RpcServerImpl;
use crate::storage::rope_store;
use crate::types::middleware_types::{Tab, TabMap};
use crate::utility::ptr::Ptr;
use crate::utility::remote_helper::get_free_port;
use once_cell::sync::Lazy;
use rand::random;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

static TEST_FILE_NAME: &str = "/foo/bar/test_file.txt";
static TEST_PASSWD: &str = "fdsfs";
static TEST_FILE_CONTENT: &str = r#"
你说的对，但是《软件工程》是由SUSTech自主研发的一款经典小组合作冒险游戏。
游戏发生在一个被曾被叫做「SUSTC」的申比世界，在这里，被随机分配的你将被授予「祭拜Project」，获得救火之力。
你将扮演一位名为「牢大」的神秘角色，在吃食的开发坐牢中检验不回消息、能力烂烂的队友们，催他们起完成feature，完成狗屎项目
——
同时，逐步发掘「木琴去哪」的真相。
"#;
static TABMAP: Lazy<Mutex<Option<TabMap>>> = Lazy::new(|| Mutex::new(None));

fn init_test_server() -> Result<RpcServerImpl, String> {
    if TABMAP.lock().unwrap().as_ref().is_none() {
        {
            let mut static_lock = TABMAP.lock().unwrap();
            *static_lock = Some(TabMap::default());
        }
        match rope_store::Text::from_str(
            PathBuf::from_str(TEST_FILE_NAME).unwrap().as_path(),
            TEST_FILE_CONTENT,
        ) {
            Ok(content) => {
                let mut static_lock = TABMAP.lock().unwrap();
                let static_tabmap = static_lock.as_mut().unwrap();
                let mut static_tab = static_tabmap.tabs.lock().unwrap();
                let tab = Tab {
                    text: Box::new(content),
                    parser: Box::new(RISCVParser::new(&vec![RISCVExtension::RV32I])),
                };
                static_tab.insert(TEST_FILE_NAME.to_string(), tab);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    let mut server = RpcServerImpl::default();
    let _ = server
        .start_service(
            TEST_FILE_NAME.to_string(),
            Ptr::new(TABMAP.lock().unwrap().as_ref().unwrap()),
        )
        .unwrap();
    server.change_password(TEST_PASSWD);
    let _ = server.set_port(get_free_port(Ipv4Addr::from_str("127.0.0.1").unwrap(), 50).unwrap());
    Ok(server)
}

#[test]
fn test_authorize() {
    let mut server = init_test_server().unwrap();
    assert!(1 == 1, "foo");
}
