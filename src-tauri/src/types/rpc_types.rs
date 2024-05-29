use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};

use serde::Deserialize;

use crate::remote::{client::RpcClientImpl, server::RpcServerImpl, ClientCursor};

#[derive(Default)]
pub struct RpcState {
    pub rpc_server: Mutex<RpcServerImpl>,
    pub rpc_client: Mutex<RpcClientImpl>,
}

#[derive(Default)]
pub struct CursorListState {
    pub cursors: Arc<Mutex<Cursor>>,
}

pub type Cursor = LinkedList<ClientCursor>;

#[derive(Clone, Deserialize)]
pub struct CursorPosition {
    pub row: u64,
    pub col: u64,
}

#[derive(Deserialize)]
pub enum FileOperation {
    Insert = 0,
    Delete = 1,
    Replace = 2,
}
