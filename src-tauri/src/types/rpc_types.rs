use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use crate::remote::{client::RpcClientImpl, server::RpcServerImpl, ClientCursor};

#[derive(Default)]
pub struct RpcState {
    pub rpc_server: Mutex<RpcServerImpl>,
    pub rpc_client: Mutex<RpcClientImpl>,
}

#[derive(Default)]
pub struct CursorListState {
    pub cursors: Arc<Mutex<CursorList>>,
}

pub type CursorList = LinkedList<ClientCursor>;

#[derive(Clone, Deserialize, Serialize)]
pub struct CursorPosition {
    pub row: u64,
    pub col: u64,
}

#[derive(Deserialize, Clone)]
pub enum FileOperation {
    Insert = 0,
    Delete = 1,
    Replace = 2,
}
